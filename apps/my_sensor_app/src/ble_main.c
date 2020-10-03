/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
//  Bluetooth LE Functions (e.g. firmware upgrade). Based on https://github.com/apache/mynewt-nimble/blob/master/apps/bleprph/src/main.c
#include "sysinit/sysinit.h"

#if MYNEWT_VAL(BLUETOOTH_LE)  //  If Bluetooth LE is enabled...
#include <assert.h>
#include <string.h>
#include <stdio.h>
#include <errno.h>
#include "os/mynewt.h"
#include "bsp/bsp.h"
#include "hal/hal_gpio.h"
#include "console/console.h"
#include "hal/hal_system.h"
#include "config/config.h"
#include "split/split.h"
#if MYNEWT_VAL(BLE_SVC_DIS_FIRMWARE_REVISION_READ_PERM) >= 0
#include "bootutil/image.h"
#include "imgmgr/imgmgr.h"
#include "services/dis/ble_svc_dis.h"
#endif

/* BLE */
#include "nimble/ble.h"
#include "host/ble_hs.h"
#include "host/util/util.h"
#include "services/gap/ble_svc_gap.h"

/* Application-specified header. */
#include "ble_prph.h"
#include "ble_peer.h"

static int bleprph_gap_event(struct ble_gap_event *event, void *arg);

//  Print info and error messages to Semihosting Console
#define MODLOG_DFLT_INFO    console_printf
#define MODLOG_DFLT_ERROR   console_printf
#define MODLOG_DFLT_FLUSH   console_flush

//  Disable info and error messages
//  #define MODLOG_DFLT_INFO(...)
//  #define MODLOG_DFLT_ERROR(...)
//  #define MODLOG_DFLT_FLUSH()

///////////////////////////////////////////////////////////////////////////////
//  Time Sync. When a BLE connection is established, we read the GATT Characteristic for the Current Time Service of the BLE Peer
//  Based on https://github.com/apache/mynewt-nimble/blob/master/apps/blecent/src/main.c

#define BLE_GATT_SVC_CTS        (0x1805)  //  GATT Service for Current Time Service
#define BLE_GATT_CHR_CUR_TIME   (0x2A2B)  //  GATT Characteristic for Current Time Service

static void blecent_read(const struct blepeer *peer);
static int blecent_on_read(uint16_t conn_handle, const struct ble_gatt_error *error, struct ble_gatt_attr *attr, void *arg);
static void print_mbuf(const struct os_mbuf *om);

/// Called when GATT Service Discovery of the BLE Peer has completed
static void blecent_on_disc_complete(const struct blepeer *peer, int status, void *arg) {
    if (status != 0) {
        //  Service discovery failed
        MODLOG_DFLT_ERROR("Error: Service discovery failed; status=%d conn_handle=%d\n", status, peer->conn_handle);
        goto err;
    }

    //  GATT Service Discovery has completed successfully.  Now we have a complete list of services, characteristics, and descriptors that the peer supports.
    MODLOG_DFLT_INFO("Service discovery complete; status=%d conn_handle=%d\n", status, peer->conn_handle);

    //  Read the GATT Characteristics from the peer
    blecent_read(peer);
    return;

err:
    //  Don't terminate the BLE connection yet, may be used by MCU Manager
    //  ble_gap_terminate(peer->conn_handle, BLE_ERR_REM_USER_CONN_TERM);
    return;
}

/// Read the GATT Characteristic for Current Time Service from the BLE Peer
static void blecent_read(const struct blepeer *peer) {
    //  Find the GATT Characteristic for Current Time Service from the discovered GATT Characteristics
    const struct blepeer_chr *chr = blepeer_chr_find_uuid(
        peer,
        BLE_UUID16_DECLARE(BLE_GATT_SVC_CTS),      //  GATT Service for Current Time Service
        BLE_UUID16_DECLARE(BLE_GATT_CHR_CUR_TIME)  //  GATT Characteristic for Current Time Service
    );
    if (chr == NULL) {
        MODLOG_DFLT_ERROR("Error: Peer doesn't support CTS\n");
        goto err;
    }

    //  Read the Current Time Service Characteristic
    int rc = ble_gattc_read(
        peer->conn_handle,      //  BLE Connection
        chr->chr.val_handle,    //  GATT Characteristic
        blecent_on_read,        //  Callback after reading
        NULL                    //  Callback argument
    );
    if (rc != 0) {
        MODLOG_DFLT(ERROR, "Error: Can't read CTS: %d\n", rc);
        goto err;
    }
    return;

err:
    //  Don't terminate the BLE connection yet, may be used by MCU Manager
    //  ble_gap_terminate(peer->conn_handle, BLE_ERR_REM_USER_CONN_TERM);
    return;
}

/// Called when Current Time Service GATT Characteristic has been read
static int blecent_on_read(uint16_t conn_handle, const struct ble_gatt_error *error, struct ble_gatt_attr *attr, void *arg) {
    //  Read the current time from the Current Time Service
    MODLOG_DFLT_INFO("Read complete; status=%d conn_handle=%d", error->status, conn_handle);
    if (error->status == 0) {
        MODLOG_DFLT_INFO(" attr_handle=%d value=", attr->handle);
        print_mbuf(attr->om);
    }
    MODLOG_DFLT_INFO("\n");

    //  TODO: Set the Mynewt system time from the current time

    //  TODO: Update the current time periodically
    return 0;
}

/// Print the mbuf
static void print_mbuf(const struct os_mbuf *om) {
    while (om != NULL) {
        console_dump(om->om_data, om->om_len);
        console_printf("\n");
        om = SLIST_NEXT(om, om_next);
    }
}

#ifdef NOTUSED
nRF Connect disconnects from PineTime...

connection established; status=0 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Remote PineTime Log
Failed to add peer; rc=6
Render LVGL display...
Render LVGL display...
disconnect; reason=574 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Render LVGL display...
connection established; status=0 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Remote PineTime Log
Failed to add peer; rc=6
Render LVGL display...
Render LVGL display...
Render LVGL display...
disconnect; reason=574 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Render LVGL display...
connection established; status=0 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Failed to add peer; rc=6
Render LVGL display...
Render LVGL display...
Render LVGL display...
disconnect; reason=574 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Render LVGL display...
connection established; status=0 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Failed to add peer; rc=6
Render LVGL display...
Remote PineTime Log
Render LVGL display...
disconnect; reason=574 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Render LVGL display...
connection established; status=0 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Remote PineTime Log
Failed to add peer; rc=6
Render LVGL display...
Render LVGL display...
Render LVGL display...
disconnect; reason=574 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Render LVGL display...
connection established; status=0 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Remote PineTime Log
Failed to add peer; rc=6
Render LVGL display...
Render LVGL display...
Render LVGL display...
Render LVGL display...
Render LVGL display...
Render LVGL display...
Remote PineTime Log
disconnect; reason=520 handle=1 our_ota_addr_type=1 our_ota_addr= our_id_addr_type=1 our_id_addr= peer_ota_addr_type=1 peer_ota_addr= peer_id_addr_type=1 peer_id_addr= conn_itvl=36 conn_latency=0 supervision_timeout=500 encrypted=0 authenticated=0 bonded=0
Render LVGL display...
shutdown command invoked

#endif  // NOTUSED

//  End of Time Sync
///////////////////////////////////////////////////////////////////////////////

/**
 * Logs information about a connection to the console.
 */
static void
bleprph_print_conn_desc(struct ble_gap_conn_desc *desc)
{
    MODLOG_DFLT_INFO("handle=%d our_ota_addr_type=%d our_ota_addr=",
                desc->conn_handle, desc->our_ota_addr.type);
    print_addr(desc->our_ota_addr.val);
    MODLOG_DFLT_INFO(" our_id_addr_type=%d our_id_addr=",
                desc->our_id_addr.type);
    print_addr(desc->our_id_addr.val);
    MODLOG_DFLT_INFO(" peer_ota_addr_type=%d peer_ota_addr=",
                desc->peer_ota_addr.type);
    print_addr(desc->peer_ota_addr.val);
    MODLOG_DFLT_INFO(" peer_id_addr_type=%d peer_id_addr=",
                desc->peer_id_addr.type);
    print_addr(desc->peer_id_addr.val);
    MODLOG_DFLT_INFO(" conn_itvl=%d conn_latency=%d supervision_timeout=%d "
                "encrypted=%d authenticated=%d bonded=%d\n",
                desc->conn_itvl, desc->conn_latency,
                desc->supervision_timeout,
                desc->sec_state.encrypted,
                desc->sec_state.authenticated,
                desc->sec_state.bonded);
}

/**
 * Enables advertising with the following parameters:
 *     o General discoverable mode.
 *     o Undirected connectable mode.
 */
static void
bleprph_advertise(void)
{
    uint8_t own_addr_type;
    struct ble_gap_adv_params adv_params;
    struct ble_hs_adv_fields fields;
    const char *name;
    int rc;

    /* Figure out address to use while advertising (no privacy for now) */
    rc = ble_hs_id_infer_auto(0, &own_addr_type);
    if (rc != 0) {
        MODLOG_DFLT_ERROR("error determining address type; rc=%d\n", rc);
        MODLOG_DFLT_FLUSH();
        return;
    }

    /**
     *  Set the advertisement data included in our advertisements:
     *     o Flags (indicates advertisement type and other general info).
     *     o Advertising tx power.
     *     o Device name.
     *     o 16-bit service UUIDs (alert notifications).
     */

    memset(&fields, 0, sizeof fields);

    /* Advertise two flags:
     *     o Discoverability in forthcoming advertisement (general)
     *     o BLE-only (BR/EDR unsupported).
     */
    fields.flags = BLE_HS_ADV_F_DISC_GEN |
                   BLE_HS_ADV_F_BREDR_UNSUP;

    /* Indicate that the TX power level field should be included; have the
     * stack fill this value automatically.  This is done by assiging the
     * special value BLE_HS_ADV_TX_PWR_LVL_AUTO.
     */
    fields.tx_pwr_lvl_is_present = 1;
    fields.tx_pwr_lvl = BLE_HS_ADV_TX_PWR_LVL_AUTO;

    name = ble_svc_gap_device_name();
    fields.name = (uint8_t *)name;
    fields.name_len = strlen(name);
    fields.name_is_complete = 1;

    fields.uuids16 = (ble_uuid16_t[]){
        BLE_UUID16_INIT(GATT_SVR_SVC_ALERT_UUID)
    };
    fields.num_uuids16 = 1;
    fields.uuids16_is_complete = 1;

    rc = ble_gap_adv_set_fields(&fields);
    if (rc != 0) {
        MODLOG_DFLT_ERROR("error setting advertisement data; rc=%d\n", rc);
        MODLOG_DFLT_FLUSH();
        return;
    }

    /* Begin advertising. */
    memset(&adv_params, 0, sizeof adv_params);
    adv_params.conn_mode = BLE_GAP_CONN_MODE_UND;
    adv_params.disc_mode = BLE_GAP_DISC_MODE_GEN;
    rc = ble_gap_adv_start(own_addr_type, NULL, BLE_HS_FOREVER,
                           &adv_params, bleprph_gap_event, NULL);
    if (rc != 0) {
        MODLOG_DFLT_ERROR("error enabling advertisement; rc=%d\n", rc);
        MODLOG_DFLT_FLUSH();
        return;
    }
}

/**
 * The nimble host executes this callback when a GAP event occurs.  The
 * application associates a GAP event callback with each connection that forms.
 * bleprph uses the same callback for all connections.
 *
 * @param event                 The type of event being signalled.
 * @param ctxt                  Various information pertaining to the event.
 * @param arg                   Application-specified argument; unuesd by
 *                                  bleprph.
 *
 * @return                      0 if the application successfully handled the
 *                                  event; nonzero on failure.  The semantics
 *                                  of the return code is specific to the
 *                                  particular GAP event being signalled.
 */
static int
bleprph_gap_event(struct ble_gap_event *event, void *arg)
{
    struct ble_gap_conn_desc desc;
    int rc;

    switch (event->type) {
    case BLE_GAP_EVENT_CONNECT:
        /* A new connection was established or a connection attempt failed. */
        MODLOG_DFLT_INFO("connection %s; status=%d ",
                    event->connect.status == 0 ? "established" : "failed",
                    event->connect.status);
        if (event->connect.status == 0) {
            rc = ble_gap_conn_find(event->connect.conn_handle, &desc);
            assert(rc == 0);
            bleprph_print_conn_desc(&desc);

#if MYNEWT_VAL(BLEPRPH_LE_PHY_SUPPORT)
            phy_conn_changed(event->connect.conn_handle);
#endif

            //  When a BLE connection is established, we read the GATT Characteristic of the Current Time Service of the BLE Peer
            //  Based on https://github.com/apache/mynewt-nimble/blob/master/apps/blecent/src/main.c

            //  Remember the BLE Peer
            rc = blepeer_add(event->connect.conn_handle);
            if (rc != 0) { MODLOG_DFLT_ERROR("Failed to add peer; rc=%d\n", rc); MODLOG_DFLT_FLUSH(); }
            else {
                //  Discover all GATT Sevices in BLE Peer (including Current Time Service)
                rc = blepeer_disc_all(event->connect.conn_handle, blecent_on_disc_complete, NULL);
                if (rc != 0) { MODLOG_DFLT_ERROR("Failed to discover services; rc=%d\n", rc); MODLOG_DFLT_FLUSH(); }
            }
        }
        MODLOG_DFLT_INFO("\n");

        if (event->connect.status != 0) {
            /* Connection failed; resume advertising. */
            bleprph_advertise();
        }
        return 0;

    case BLE_GAP_EVENT_DISCONNECT:
        MODLOG_DFLT_INFO("disconnect; reason=%d ", event->disconnect.reason);
        bleprph_print_conn_desc(&event->disconnect.conn);
        MODLOG_DFLT_INFO("\n");

#if MYNEWT_VAL(BLEPRPH_LE_PHY_SUPPORT)
        phy_conn_changed(CONN_HANDLE_INVALID);
#endif

        /* Connection terminated; resume advertising. */
        bleprph_advertise();
        MODLOG_DFLT_FLUSH();
        return 0;

    case BLE_GAP_EVENT_CONN_UPDATE:
        /* The central has updated the connection parameters. */
        MODLOG_DFLT_INFO("connection updated; status=%d ",
                    event->conn_update.status);
        rc = ble_gap_conn_find(event->connect.conn_handle, &desc);
        assert(rc == 0);
        bleprph_print_conn_desc(&desc);
        MODLOG_DFLT_INFO("\n");
        MODLOG_DFLT_FLUSH();
        return 0;

    case BLE_GAP_EVENT_ADV_COMPLETE:
        MODLOG_DFLT_INFO("advertise complete; reason=%d",
                    event->adv_complete.reason);
        bleprph_advertise();
        MODLOG_DFLT_FLUSH();
        return 0;

    case BLE_GAP_EVENT_ENC_CHANGE:
        /* Encryption has been enabled or disabled for this connection. */
        MODLOG_DFLT_INFO("encryption change event; status=%d ",
                    event->enc_change.status);
        rc = ble_gap_conn_find(event->connect.conn_handle, &desc);
        assert(rc == 0);
        bleprph_print_conn_desc(&desc);
        MODLOG_DFLT_INFO("\n");
        MODLOG_DFLT_FLUSH();
        return 0;

    case BLE_GAP_EVENT_SUBSCRIBE:
        MODLOG_DFLT_INFO("subscribe event; conn_handle=%d attr_handle=%d "
                          "reason=%d prevn=%d curn=%d previ=%d curi=%d\n",
                    event->subscribe.conn_handle,
                    event->subscribe.attr_handle,
                    event->subscribe.reason,
                    event->subscribe.prev_notify,
                    event->subscribe.cur_notify,
                    event->subscribe.prev_indicate,
                    event->subscribe.cur_indicate);
        MODLOG_DFLT_FLUSH();
        return 0;

    case BLE_GAP_EVENT_MTU:
        MODLOG_DFLT_INFO("mtu update event; conn_handle=%d cid=%d mtu=%d\n",
                    event->mtu.conn_handle,
                    event->mtu.channel_id,
                    event->mtu.value);
        MODLOG_DFLT_FLUSH();
        return 0;

    case BLE_GAP_EVENT_REPEAT_PAIRING:
        /* We already have a bond with the peer, but it is attempting to
         * establish a new secure link.  This app sacrifices security for
         * convenience: just throw away the old bond and accept the new link.
         */

        /* Delete the old bond. */
        rc = ble_gap_conn_find(event->repeat_pairing.conn_handle, &desc);
        assert(rc == 0);
        ble_store_util_delete_peer(&desc.peer_id_addr);

        /* Return BLE_GAP_REPEAT_PAIRING_RETRY to indicate that the host should
         * continue with the pairing operation.
         */
        MODLOG_DFLT_FLUSH();
        return BLE_GAP_REPEAT_PAIRING_RETRY;

#if MYNEWT_VAL(BLEPRPH_LE_PHY_SUPPORT)
    case BLE_GAP_EVENT_PHY_UPDATE_COMPLETE:
        /* XXX: assume symmetric phy for now */
        phy_update(event->phy_updated.tx_phy);
        MODLOG_DFLT_FLUSH();
        return 0;
#endif
    }

    return 0;
}

static void
bleprph_on_reset(int reason)
{
    MODLOG_DFLT_ERROR("Resetting state; reason=%d\n", reason);
    MODLOG_DFLT_FLUSH();
}

static void
bleprph_on_sync(void)
{
    int rc;

    /* Make sure we have proper identity address set (public preferred) */
    rc = ble_hs_util_ensure_addr(0);
    assert(rc == 0);

    /* Begin advertising. */
    bleprph_advertise();
}

/**
 * main
 *
 * The main task for the project. This function initializes the packages,
 * then starts serving events from default event queue.
 *
 * @return int 0
 */
int
start_ble(void)
{
    MODLOG_DFLT_INFO("Starting BLE...\n");
    MODLOG_DFLT_FLUSH();

#if MYNEWT_VAL(BLE_SVC_DIS_FIRMWARE_REVISION_READ_PERM) >= 0
    struct image_version ver;
    static char ver_str[IMGMGR_NMGR_MAX_VER];
#endif
    int rc;

    /* Initialize the NimBLE host configuration. */
    ble_hs_cfg.reset_cb = bleprph_on_reset;
    ble_hs_cfg.sync_cb = bleprph_on_sync;
    ble_hs_cfg.gatts_register_cb = gatt_svr_register_cb;
    ble_hs_cfg.store_status_cb = ble_store_util_status_rr;

    rc = gatt_svr_init();
    assert(rc == 0);

    /* Set the default device name. */
    rc = ble_svc_gap_device_name_set("pinetime");
    assert(rc == 0);

#if MYNEWT_VAL(BLE_SVC_DIS_FIRMWARE_REVISION_READ_PERM) >= 0
    /* Set firmware version in DIS */
    imgr_my_version(&ver);
    imgr_ver_str(&ver, ver_str);
    ble_svc_dis_firmware_revision_set(ver_str);
#endif

#if MYNEWT_VAL(BLEPRPH_LE_PHY_SUPPORT)
    phy_init();
#endif

    conf_load();

    /* If this app is acting as the loader in a split image setup, jump into
     * the second stage application instead of starting the OS.
     */
#if MYNEWT_VAL(SPLIT_LOADER)
    {
        void *entry;
        rc = split_app_go(&entry, true);
        if (rc == 0) {
            hal_system_start(entry);
        }
    }
#endif

    //  Init Service Discovery
    rc = blepeer_init();
    assert(rc == 0);

    MODLOG_DFLT_INFO("BLE started\n");
    MODLOG_DFLT_FLUSH();
    return 0;
}

#else //  If Bluetooth LE is disabled...

int start_ble(void) {
    //  Bluetooth LE not supported.
    return 0;
}
#endif  //  MYNEWT_VAL(BLUETOOTH_LE)