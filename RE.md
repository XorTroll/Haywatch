# LS02 reverse-engineering

Note that this is the result of RE-ing the (unofficial) [Hello Haylou](https://play.google.com/store/apps/details?id=hu.tiborsosdevs.haylou.hello&hl=es_419&gl=US) app (reasons of this explained below).

## Hello Haylou

The official [Haylou](https://play.google.com/store/apps/details?id=com.liesheng.haylou&hl=es_419&gl=US) app, the one expected to be used with this watch, is quite bad (just check reviews/comments on the provided Play Store link). Furthermore, the core sources of the app, the vast majority of its code, is native, which complicates the RE process.

Therefore, one of the best alternatives (might be the only one?) is the mentioned `Hello Haylou` app. This app appears to be developed by a Hungarian dev team named [Tibor Borsos](https://play.google.com/store/apps/developer?id=Tibor+Borsos&hl=es_419&gl=US), who have also developed similar apps for other smart watches.

There is evidence of all the claims made on the app's decompiled code: some debug logs contain Hungarian text, and several class/type names start by `MiBand*`, suggesting this app's source might contain copy-pastes of another app supporting Xiaomi Mi Band smart-watches, [which they have actually made](https://play.google.com/store/apps/details?id=hu.tiborsosdevs.mibandage&hl=es_419&gl=US).

It's noteworthy to mention that the app supports 10 different Haylou smart watch models, but this RE-ing project is exclusively focused on the `LS02` model.

The app seems to be developed on Kotlin (aside external libs used within it), and it appears to be the result of their own RE-ing of the watch and the official app.

The dev team use their own servers (`https://hello-haylou.web.app/`) for storing watch firmware files and images. However, this isn't something really useful for this project since `LS02` watches do not support custom watch-face firmware, thus have no files for them in this servers.

Some parts of the app suggest that they have to RE the watch/the official app. First of all, many responses sent by the watch are ignored or only small parts of their contents are actually used. Furthermore, commands are often constructed by taking base byte-array (which appears to be a dump of an actual command) and modify the key parts of it.

## Watch features

> TODO

## Device communication

Communications with the corresponding device are a key component of part of the watch's features.

The watch's UI options are severely limited, this being particularly clear when compared to the options the device is expected to request and/or receive from it.

The watch can communicate through BLE/GATT functionality. While the watch expects to connect to a mobile phone (furthermore, it expects and encourages the user to use the official app) any BLE-capable device can do so.

Watch recognition is just based on the device's name, since all `LS02` watches appear as Bluetooth devices named "Haylou Smart Watch 2". The app simply tries to connect with devices with that exact name (and the same goes for the other supported watch models).

These are all the available GATT characteristics:

```sh
Characteristic { uuid: 00002a05-0000-1000-8000-00805f9b34fb, service_uuid: 00001801-0000-1000-8000-00805f9b34fb, properties: INDICATE }
Characteristic { uuid: 00002a19-0000-1000-8000-00805f9b34fb, service_uuid: 0000180f-0000-1000-8000-00805f9b34fb, properties: READ | NOTIFY }
Characteristic { uuid: 000033f1-0000-1000-8000-00805f9b34fb, service_uuid: 000055ff-0000-1000-8000-00805f9b34fb, properties: READ | WRITE }
Characteristic { uuid: 000033f2-0000-1000-8000-00805f9b34fb, service_uuid: 000055ff-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
Characteristic { uuid: 000034f1-0000-1000-8000-00805f9b34fb, service_uuid: 000056ff-0000-1000-8000-00805f9b34fb, properties: READ | WRITE_WITHOUT_RESPONSE }
Characteristic { uuid: 000034f2-0000-1000-8000-00805f9b34fb, service_uuid: 000056ff-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
Characteristic { uuid: 00006001-0000-1000-8000-00805f9b34fb, service_uuid: 000060ff-0000-1000-8000-00805f9b34fb, properties: READ | WRITE_WITHOUT_RESPONSE }
Characteristic { uuid: 00006002-0000-1000-8000-00805f9b34fb, service_uuid: 000060ff-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
Characteristic { uuid: 00006101-0000-1000-8000-00805f9b34fb, service_uuid: 000061ff-0000-1000-8000-00805f9b34fb, properties: READ | WRITE_WITHOUT_RESPONSE }
Characteristic { uuid: 00006102-0000-1000-8000-00805f9b34fb, service_uuid: 000061ff-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
Characteristic { uuid: 0000b003-0000-1000-8000-00805f9b34fb, service_uuid: 000055ff-0000-1000-8000-00805f9b34fb, properties: READ | WRITE }
Characteristic { uuid: 0000b004-0000-1000-8000-00805f9b34fb, service_uuid: 000055ff-0000-1000-8000-00805f9b34fb, properties: NOTIFY }
Characteristic { uuid: 0000ffd1-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: WRITE_WITHOUT_RESPONSE }
Characteristic { uuid: 0000ffd2-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: READ }
Characteristic { uuid: 0000ffd3-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: READ }
Characteristic { uuid: 0000ffd4-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: READ }
Characteristic { uuid: 0000ffd5-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: READ }
Characteristic { uuid: 0000ffd8-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: WRITE_WITHOUT_RESPONSE }
Characteristic { uuid: 0000ffe0-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: READ }
Characteristic { uuid: 0000fff1-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: READ }
Characteristic { uuid: 0000fff2-0000-1000-8000-00805f9b34fb, service_uuid: 0000d0ff-3c17-d293-8e48-14fe2e4da212, properties: WRITE }
```

The following GATT characteristics are used for writing requests:

- Characteristic of UUID `000033F1-0000-1000-8000-00805F9B34FB` and service UUID `000055FF-0000-1000-8000-00805F9B34FB`, which we will refer to as `CHAR_GENERAL_RW_1`

- Characteristic of UUID `00006001-0000-1000-8000-00805F9B34FB` and service UUID `000060FF-0000-1000-8000-00805F9B34FB`, which we will refer to as `CHAR_DATA2_RW`

The following GATT characteristics are used for getting response notifications:

- Characteristic of UUID `000033F2-0000-1000-8000-00805F9B34FB` and service UUID `000055FF-0000-1000-8000-00805F9B34FB`, which we will refer to as `CHAR_GENERAL_N_1`

- Characteristic of UUID `00006002-0000-1000-8000-00805F9B34FB` and service UUID `000060FF-0000-1000-8000-00805F9B34FB`, which we will refer to as `CHAR_DATA2_N`

Note that write-characteristics (and the rest probably too...?) appear to have a limit of max. 48 bytes.

Other characteristics might also be used/usable, but their purposes are still unknown. For example, one can try listening to other notification characteristics different from `CHAR_GENERAL_N_1` after writing to `CHAR_GENERAL_RW_1`, and different responses might be received.

### General command structure

We will distinguish between *request* and *response* commands, where the former ones are written on a certain write-characteristic and the latter ones are notified on (and thus received from) a certain notify-characteristic.

In both cases, however, the general structure is the same. The commands always start with an initial `u8` ID which identifies them. Considering how GATT works, it's necessary for both requests and responses to be adequately identified.

Note that the command/field names used from now on are completely guessed, and the result of both leftover names of the RE-d app, texts from actual watch features and my own deductions of command features.

## Requests/responses via `CHAR_GENERAL_RW_1` and `CHAR_GENERAL_N_1`

The requests described in this section are all sent via `CHAR_GENERAL_RW_1`.
The responses described in this section are all received via `CHAR_GENERAL_N_1`.

TODO: quitar referencias repetidas a los chars

### Pairing

All pairing-related request/response commands start the following way:

| Offset | Type + name    | Description       |
|--------|----------------|-------------------|
| 0x00   | u8 id          | Command ID (0x20) |
| 0x01   | u8 op          | Operation type    |

After that commands may contain more data.

Pairing request commands have the following structure:

| Offset | Type + name    | Description                          |
|--------|----------------|--------------------------------------|
| 0x00   | u8 id          | Command ID (0x20)                    |
| 0x01   | u8 op          | Operation type (request pair = 0x02) |
| 0x02   | u8[4] pair_key | 4-number pair key                    |

This commands are written via `CHAR_GENERAL_RW_1`.

Note that, without sending any key bytes or in general sending less than 4 key-bytes, the watch seems to treat the unset key bytes as `0xFF` (the way to get the watch's current pair key is detailed below).
Any extra bytes are ignored, but unlike with other commands, the command is accepted.

Watches may be connected to different devices without having to re-pair, as long as the pair key is the same.

The watch will prompt a dialog for the user to accept the device pairing (if no pairing exists or if pairing with a different pair key), and the following command will be received at the same time via `CHAR_GENERAL_N_1`:

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x20) |
| 0x01   | u8 unk_1    | Unknown (0x04)    |
| 0x02   | u8 unk_2    | Unknown (0x04)    |

If the pairing dialog is accepted the following response commands will be received in order via `CHAR_GENERAL_N_1`:

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x33) |
| 0x01   | u8 unk_1    | Unknown (0x04)    |
| 0x02   | u8 unk_2    | Unknown (0x02)    |

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x33) |
| 0x01   | u8 unk_1    | Unknown (0x04)    |
| 0x02   | u8 unk_2    | Unknown (0x01)    |

Furthermore, the watch may also send its current battery level via `CHAR_GENERAL_N_1` after successfully pairing (battery response commands are detailed below).

If the pairing dialog is rejected instead, the following response command will be received via `CHAR_GENERAL_N_1`:

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x33) |
| 0x01   | u8 unk_1    | Unknown (0x04)    |
| 0x02   | u8 unk_2    | Unknown (0x05)    |

However, if the watch is already paired with the requested pair key, the following response command will be received in order via `CHAR_GENERAL_N_1` and nothing else will change (this way one ensures that the watch is actually paired, otherwise the dialog would prompt):

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x1C) |

If the paired device was disconnected instead, the following response command will be received in order via `CHAR_GENERAL_N_1` after pairing:

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x33) |
| 0x01   | u8 unk_1    | Unknown (0x04)    |
| 0x02   | u8 unk_2    | Unknown (0x01)    |

In the scenario where the watch was reset (that is, no device is paired with it), trying unk value `0x00` or any value higher or equal to `0x04` in the pairing request resulted in the watch sending the mentioned already-paired `0x1C` command, but the watch did nothing and still was showing the initial instructions to pair.

### Pair key

The following request command may be sent via `CHAR_GENERAL_RW_1` to request the current pair key:

| Offset | Type + name | Description                                      |
|--------|-------------|--------------------------------------------------|
| 0x00   | u8 id       | Command ID (0x20)                                |
| 0x01   | u8 op       | Operation type (request current pair key = 0x03) |

After sending said request command the following response command will be received via `CHAR_GENERAL_N_1`:

| Offset | Type + name        | Description                                      |
|--------|--------------------|--------------------------------------------------|
| 0x00   | u8 id              | Command ID (0x20)                                |
| 0x01   | u8 op              | Operation type (request current pair key = 0x03) |
| 0x02   | u8[4] cur_pair_key | Current 4-number pair key                        |

Note that, inmediately after the initial pairing is done, sending certain command requests will make the watch display a "device successfully connected" screen. For instance, getting the watch's battery does not prompt this, but setting time/date or getting the watch's firmware (all these command requests are detailed below) does.

Furthermore, this even happens with some invalid command requests, so there might be a certain list of command IDs that are treated by the watch as the kind of commands a device will send inmediately after pairing that guarantee that the pairing was successful on both ends.

### Reset

To unpair/reset, send the following command request via `CHAR_GENERAL_RW_1`:

| Offset | Type + name | Description        |
|--------|-------------|--------------------|
| 0x00   | u8 id       | Command ID (0x07)  |
| 0x01   | u8 opt      | Option (see below) |

Option `0x00` will reset the device (same as the reset on the watch settings, which will obviously unpair and reboot the watch). Option `0x01` appears to do the same but powering-off the watch instead of rebooting it, so that after turning it on the watch will be reset.

Requesting this command (with any of the options) when the watch is already unpaired (when it's on the initial screen) does nothing. In fact, no requests seem to do anything in this state other than the pairing request, so from now on all the command bevahior will assume that the watch is already paired with a device, with all its functionalities.

### Battery

Battery request commands have the following structure:

| Offset | Type + name | Description        |
|--------|-------------|--------------------|
| 0x00   | u8 id       | Command ID (0xA2)  |

Battery response commands have the following structure:

| Offset | Type + name | Description               |
|--------|-------------|---------------------------|
| 0x00   | u8 id       | Command ID (0xA2)         |
| 0x01   | u8 battery  | Battery charge percentage |

One may send the command request via `CHAR_GENERAL_RW_1`, and the watch will send this response via `CHAR_GENERAL_N_1`.

Note that, when the watch battery changes (when the percentage drops), the watch will by itself send a battery response via `CHAR_GENERAL_N_1`, with the aim of notifying the device that the battery level changed.

As mentioned above, after an initial pair the watch will (only sometimes?) also send by itself a battery response via `CHAR_GENERAL_N_1`.

### Firmware

Firmware request commands have the following structure:

| Offset | Type + name | Description        |
|--------|-------------|--------------------|
| 0x00   | u8 id       | Command ID (0xA1)  |

Firmware response commands have the following structure:

| Offset | Type + name  | Description       |
|--------|--------------|-------------------|
| 0x00   | u8 id        | Command ID (0xA1) |
| 0x01   | char[] fw  | Firmware string   |

One may send the command request via `CHAR_GENERAL_RW_1`, and the watch will send this response via `CHAR_GENERAL_N_1`.

It's unclear whether firmware strings have a fixed length, but some of the app server contents for other watch models (which contain their version strings, similar to this model's ones) suggest that they may not have a fixed length.

An example of a firmware string is `RH208DV000924`, where `V000924` corresponds to the firmware version, the same one the watch displays on the "about" menu.

### Date and time

Commands to set the watch's time/date have the following structure:

| Offset | Type + name | Description       |
|--------|-------------|-------------------|
| 0x00   | u8 id       | Command ID (0x04) |
| 0x01   | u16 year_be | Year (big-endian) |
| 0x03   | u8 month    | Month             |
| 0x04   | u8 day      | Day               |
| 0x05   | u8 hour     | Hour              |
| 0x06   | u8 min      | Minutes           |
| 0x07   | u8 sec      | Seconds           |

Both request and response commands have the same format. One may send this command request via `CHAR_GENERAL_RW_1`, and the watch will send its response via `CHAR_GENERAL_N_1`. The response will be the same as the request, as some sort of confirmation that date/time were set correctly.

The watch itself computes the corresponding day of the week when displaying the date.

The watch starts by setting its date to Jan 1st 2020 and its date to `08:00:00` right after the initial pairing, and it's expected for the paired device to inmediately set the current date/time. Note that not all the available watch faces display the full date/time. In fact, the only known way of knowing the current year for the watch is viewing it on the few watch faces which actually display it.

### Pulses

Pulses (pulse commands) are the way the watch sends requests to the device as responses.

Response commands sending pulses have the following structure:

| Offset | Type + name | Description            |
|--------|-------------|------------------------|
| 0x00   | u8 id       | Command ID (0xD1)      |
| 0x01   | u8 pulse    | Pulse type (see below) |

The watch will send this pulse requests as responses via `CHAR_GENERAL_N_1` when necessary.

The following pulse types are known:

- Value `0x02` corresponds to the user clicking the "hang call" button on a call alert message (alert messages are detailed below), telling the device to hang the current call.

- Values `0x07`, `0x08` and `0x09` correspond to pausing/resuming the current music, skipping to the next song and rewinding/moving back to the previous song respectively, all user-inputs from the music menu on the watch.

- Value `0x0A` corresponds to the device-finding option, where the watch "rings" the device.

### Display formats

The following command is used to set the watch's time and distance units (for formatting):

| Offset | Type + name      | Description                                                           |
|--------|------------------|-----------------------------------------------------------------------|
| 0x00   | u8 id            | Command ID (0x01)                                                     |
| 0x01   | u8 distance_unit | Distance unit type (`0x01` = metric system, `0x02` = imperial system) |
| 0x02   | u8 time_unit     | Time unit type (`0x01` = 24h format, `0x02` = 12h format)             |

This request command may be sent via `CHAR_GENERAL_RW_1`, and the watch will respond with the same command (again, as some sort of confirmation) via `CHAR_GENERAL_N_1`.

Note that the distance unit is only used to display the currently walked distance (either in kilometers or miles) and the time format unit is used to display the current time.

### Alert messages

The watch supports receiving messages from the paired device. This messages are meant to be for notifications received from the device's different messaging/social media applications, etc.

This is currently the only known request type that suffers from the max. write/command size limitation, thus alert messages have to be split into at least two request commands.

The text message is sent through various request commands of similar structure, which we will refer to as "batches".

The first request command (message batch) has the following structure:

| Offset | Type + name             | Description                                              |
|--------|-------------------------|----------------------------------------------------------|
| 0x00   | u8 id                   | Command ID (0x0F)                                        |
| 0x01   | u8 batch_idx            | Index of the current batch (`0x00` for this first batch) |
| 0x02   | u8 type                 | Alert type (see below)                                   |
| 0x03   | u8 msg_len              | Length (in bytes) of the entire message in UTF16-BE      |
| 0x04   | u16[] msg_batch_utf16be | First batch of the message in UTF16-BE                   |

The following (optional) message batches have the following structure:

| Offset | Type + name             | Description                                       |
|--------|-------------------------|---------------------------------------------------|
| 0x00   | u8 id                   | Command ID (0x0F)                                 |
| 0x01   | u8 batch_idx            | Index of the current batch (`0x01`, `0x02`, etc.) |
| 0x02   | u16[] msg_batch_utf16be | Current batch of the message in UTF16-BE          |

Note that the size of each batch's message data is arbitrary, as long as all the batches end up containing the full message size sent in the first one (I guess). For instance, the RE-d app builds these batches so that no command exceeds 20 bytes in size, but as long as the commented max. size is not surpassed, longer batches/commands can be made. (maybe the app uses 20 as the command limit instead of the actual 48 because older watch hardware/software versions may have had that limit, in order to ensure compatibility with all `LS02` hardware/software versions?)

After sending a message batch command request via `CHAR_GENERAL_RW_1`, the following response is received via `CHAR_GENERAL_N_1` (again, as some sort of confirmation):

| Offset | Type + name  | Description                                    |
|--------|--------------|------------------------------------------------|
| 0x00   | u8 id        | Command ID (0x0F)                              |
| 0x01   | u8 batch_idx | Index of the sent batch (`0x01`, `0x02`, etc.) |

After having split the UTF16-BE message text and sent it to the watch into all the needed batches via `CHAR_GENERAL_RW_1`, the finalization command is sent via `CHAR_GENERAL_RW_1` too, essentially telling the watch that all the alert message batches are already sent:

| Offset | Type + name | Description                               |
|--------|-------------|-------------------------------------------|
| 0x00   | u8 id       | Command ID (0x0F)                         |
| 0x01   | u8 unk      | Unknown (indicator of finalization, 0xFD) |

After sending this request command, the watch will display the alert message for a short period of time (until the screen itself blocks, so the actual screen show time?), and (unless the "trash" button is clicked) the message will get logged in a "notice" list accessible from the watch menu.

After sending this finalization command request via `CHAR_GENERAL_RW_1`, the following response is received via `CHAR_GENERAL_N_1` (again, as some sort of confirmation):

| Offset | Type + name  | Description                                      |
|--------|--------------|--------------------------------------------------|
| 0x00   | u8 id        | Command ID (0x0F)                                |
| 0x01   | u8 unk       | Unknown (indicator of finalization, 0xFD)        |
| 0x02   | u8 unk_2     | Unknown (0x00, might be padding...?)             |
| 0x03   | u8 msg_len   | Length (in bytes) of the entire received message |

Note that this last command shows that only batches from `0x00` to `0xFC` are allowed (thus 253 max. batches, the first one and 252 other), and by the max. size write/command limitation (which leaves us with a max. text length of 22 characters for the first batch and 23 for the rest) that yields us a theoretical total max. message length of 5818 characters.

However, the watch is not really prepared to show particularly large message texts, and it appears to effectively get/use only the first 160 characters (?).

These are the supported alert message types (most of them are for different messaging/social media applications):

| Value | Description                               |
|-------|-------------------------------------------|
| 0x00  | Phone call (a call started on the device) |
| 0x01  | QQ                                        |
| 0x02  | WeChat                                    |
| 0x04  | Message (generic message, SMS, etc.)      |
| 0x05  | Facebook                                  |
| 0x06  | Twitter                                   |
| 0x07  | WhatsApp                                  |
| 0x08  | Skype                                     |
| 0x09  | Messenger                                 |
| 0x0A  | Hangouts                                  |
| 0x0B  | LINE                                      |
| 0x0C  | LinkedIn                                  |
| 0x0D  | Instagram                                 |
| 0x0E  | Viber                                     |
| 0x0F  | KakaoTalk                                 |
| 0x10  | VK                                        |
| 0x11  | Snapchat                                  |
| 0x12  | Google+                                   |
| 0x13  | e-mail (might be just Gmail)              |
| 0x14  | Flickr                                    |
| 0x15  | Tumblr                                    |
| 0x16  | Pinterest                                 |
| 0x17  | YouTube                                   |

For alert type `0x03` surprisingly nothing happens (no notification is shown nor logged).

For alert types higher than `0x17` no notification is shown, but a message with no icon does get logged.

The call alert is a special one, where the message is likely meant to be the caller name/contact. It covers the entire screen and a button for hanging up the call is shown (pressing it will send a pulse as explained above). Even if the screen automatically locks the alert-screen persists, but pressing the call-end button will close it and sent the mentioned pulse. Actually, pressing the watch's button will also close it but won't send the pulse whatsoever (is this intended at all...?)

### Weather

The watch supports displaying weather information for the current day and the following three days.

The following request command can be sent via `CHAR_GENERAL_RW_1` to set the current day's weather info:

| Offset | Type + name | Description                       |
|--------|-------------|-----------------------------------|
| 0x00   | u8 id       | Command ID (0x11)                 |
| 0x01   | u8 date     | Weather date (current day = 0x01) |
| 0x02   | u8 type     | Weather type (see below)          |
| 0x03   | u8 unk      | Unknown value (0x00)              |
| 0x04   | u8 cur_temp | Current temperature               |
| 0x05   | u8 max_temp | Maximum temperature               |
| 0x06   | u8 min_temp | Minimum temperature               |

The following request command can be sent via `CHAR_GENERAL_RW_1` to set the following days' weather info:

| Offset | Type + name    | Description                                    |
|--------|----------------|------------------------------------------------|
| 0x00   | u8 id          | Command ID (0x11)                              |
| 0x01   | u8 date        | Weather date (following days = 0x02)           |
| 0x02   | u8 d1_type     | Weather type for the 1st following day         |
| 0x03   | u8 d1_unk      | Unknown value for the 1st following day (0x00) |
| 0x04   | u8 d1_max_temp | Maximum temperature for the 1st following day  |
| 0x05   | u8 d1_min_temp | Minimum temperature for the 1st following day  |
| 0x06   | u8 d2_type     | Weather type for the 2nd following day         |
| 0x07   | u8 d2_unk      | Unknown value for the 2nd following day (0x00) |
| 0x08   | u8 d2_max_temp | Maximum temperature for the 2nd following day  |
| 0x09   | u8 d2_min_temp | Minimum temperature for the 2nd following day  |
| 0x0A   | u8 d3_type     | Weather type for the 3rd following day         |
| 0x0B   | u8 d3_unk      | Unknown value for the 3rd following day (0x00) |
| 0x0C   | u8 d3_max_temp | Maximum temperature for the 3rd following day  |
| 0x0D   | u8 d3_min_temp | Minimum temperature for the 3rd following day  |

Note that all weather temperatures are in Celsius degrees.

These are the supported weather types:

| Value | Description                                |
|-------|--------------------------------------------|
| 0x01  | Sunny                                      |
| 0x02  | Sunny and cloudy                           |
| 0x03  | Sunny and rainy                            |
| 0x04  | Stormy                                     |
| 0x05  | Rainy                                      |
| 0x06  | Slightly rainy                             |
| 0x07  | Very rainy                                 |
| 0x08  | Snowy                                      |
| 0x09  | 'S'? (might be a kind of snowy weather...) |
| 0x0A  | Foggy                                      |
| 0x0B  | Windy (maybe even tornadoes?)              |
| 0x0C  | Night (clear)                              |
| 0x0D  | Cloudy night                               |
| 0x0E  | Rainy night                                |

For weather type `0x00` the watch ignores all the temperatures and sets the weather to it's default "unknown" state.

For weather types higher than `0x0E` the watch displays invalid graphics data but loads temperatures fine, likely being caused by out-of-bounds memory reading of weather graphics data.

For both requests commands, the watch responds via `CHAR_GENERAL_N_1` with the following command:

| Offset | Type + name | Description                    |
|--------|-------------|--------------------------------|
| 0x00   | u8 id       | Command ID (0x11)              |
| 0x01   | u8 date     | Weather date (the request one) |

In fact, even for requests with invalid dates (values like `0x00`, `0x03`, etc.) the watch does nothing but this response command gets sent with the corresponding request date value.

### User information

The following request command can be sent via `CHAR_GENERAL_RW_1` to set the user's information: 

| Offset | Type + name                    | Description                           |
|--------|--------------------------------|---------------------------------------|
| 0x00   | u8 id                          | Command ID (0x05)                     |
| 0x01   | u8 unk0                        | Unknown (0x00)                        |
| 0x02   | u8 height_cm                   | User's height in centimetres          |
| 0x03   | u8 unk1                        | Unknown (0x00)                        |
| 0x04   | u8 weight_kg                   | User's weight in kilograms            |
| 0x05   | u8 screen_show_timeout_seconds | Screen showtime in seconds            |
| 0x06   | u8 unk2                        | Unknown (0x00)                        |
| 0x07   | u8 unk3                        | Unknown (0x00)                        |
| 0x08   | u16 step_goal_be               | Step count goal in a day (big-endian) |
| 0x0A   | u8 lift_wrist_mode             | Lift wrist mode (see below)           |
| 0x0B   | u8 unk4                        | Unknown (0xA0)                        |
| 0x0C   | u8 unk5                        | Unknown (0x00)                        |
| 0x0D   | u8 age                         | User age                              |
| 0x0E   | u8 gender                      | User gender (see below)               |
| 0x0F   | u8 unk6                        | Unknown (0x00)                        |
| 0x10   | u8 unk7                        | Unknown (0x01)                        |
| 0x11   | u8 unk8                        | Unknown (0x01)                        |
| 0x12   | u8 unk9                        | Unknown (0x28)                        |

Lift wrist mode values are `0x00` for it being off and `0x01` for it being on (being on means that the watch will show its screen when the user moves their wrist, like the usual way moves it to look at the time, etc.)

Gender values are `0x01` for male and `0x02` for female.

All the unknown values are the ones used in the Hello Haylou application.

For this request, the watch doesn't seem to send any response, at least via `CHAR_GENERAL_N_1`.

| Offset | Type + name | Description                    |
|--------|-------------|--------------------------------|
| 0x00   | u8 id       | Command ID (0x11)              |
| 0x01   | u8 date     | Weather date (the request one) |

### Recorded steps

One can request the watch for it to send (via responses) all the information it contains about the steps the user has made; this can be done by sending the following request via `CHAR_GENERAL_RW_1`:

| Offset | Type + name | Description           |
|--------|-------------|-----------------------|
| 0x00   | u8 id       | Command ID (0xB2)     |
| 0x01   | u8 sub_id   | Command sub-ID (0x03) |
| 0x02   | u8 unk      | Unknown (0x01)        |

After sending this request, the watch will respond via `CHAR_GENERAL_N_1` with a several of the following responses:

| Offset | Type + name                | Description                                       |
|--------|----------------------------|---------------------------------------------------|
| 0x00   | u8 id                      | Command ID (0xB2)                                 |
| 0x01   | u16 date_year_be           | Year (big-endian)                                 |
| 0x03   | u8 date_month              | Month                                             |
| 0x04   | u8 date_day                | Day                                               |
| 0x05   | u8 hour                    | Hour                                              |
| 0x06   | u16 new_all_step_count_be  | Total new step count (big-endian)                 |
| 0x08   | u8 unk_1                   | Unknown                                           |
| 0x09   | u8 last_new_run_step_min   | Last minute where new running steps were recorded |
| 0x0A   | u8 unk_2                   | Unknown                                           |
| 0x0B   | u16 new_run_step_count_be  | New running step count (big-endian)               |
| 0x0D   | u8 unk_3                   | Unknown                                           |
| 0x0E   | u8 last_new_walk_step_min  | Last minute where new walking steps were recorded |
| 0x0F   | u8 unk_4                   | Unknown                                           |
| 0x10   | u16 new_walk_step_count_be | New walking step count (big-endian)               |

The watch records new given steps of two kinds: walking steps and running steps, depending on whether it detects the user walking or running (presumably higher activity/pulse). The first u16 contains their sum, and the other two u16s contain the separate counts.

A response like this is sent for each recorded hour of each recorded day (only for hours where new steps were registered).

Finally, a special "end" response is sent, indicating the end of the recorded steps responses:

| Offset | Type + name | Description           |
|--------|-------------|-----------------------|
| 0x00   | u8 id       | Command ID (0xB2)     |
| 0x01   | u8 sub_id   | Command sub-ID (0xFD) |
| 0x02   | u8 unk      | Unknown               |

### Heart rate

The watch might periodically send respondes regarding the user's heart rate.

When entering/leaving the "heart rate menu" from the watch screen, the watch will send the following response:

| Offset | Type + name | Description                |
|--------|-------------|----------------------------|
| 0x00   | u8 id       | Command ID (0xE5)          |
| 0x01   | u8 sub_id   | Command sub-ID (see below) |

The sub-ID depends on whether the user is entering (`0x11`) or leaving (`0x00`) the menu.

For some reason the watch sometimes only sends this response when entering the menu and not when leaving it. In fact, when leaving the menu the watch usually sends the following alternative response:

| Offset | Type + name   | Description           |
|--------|---------------|-----------------------|
| 0x00   | u8 id         | Command ID (0xE5)     |
| 0x01   | u8 sub_id     | Command sub-ID (0x00) |
| 0x02   | u8 unk        | Unknown               |
| 0x03   | u8 heart_rate | Heart rate in BPM     |

While in the menu, the watch periodically (about once per second) sends the following periodic response:

| Offset | Type + name   | Description           |
|--------|---------------|-----------------------|
| 0x00   | u8 id         | Command ID (0xE5)     |
| 0x01   | u8 sub_id     | Command sub-ID (0x11) |
| 0x02   | u8 unk        | Unknown               |
| 0x03   | u8 heart_rate | Heart rate            |

Again, in these two last responses, the sub-ID values also follow the same value convention of being in the menu or leaving it.

> Note: All heart rate values used/sent by the watch are in BPM, and a value of `0xFF` corresponds to an invalid/empty/not present heart rate value

### Silent mode

The watch has a "silent mode" funcionality, which can be toggled on/off in the watch menu (specifically, from the moon-shaped icon). When toggled on, the wrist mode is ignored and the watch screen won't turn on when moving your wrist, and messages/alerts aren't received nor logged whatsoever.

When the mode is toggled on/off, the watch sends the following response:

| Offset | Type + name | Description                    |
|--------|-------------|--------------------------------|
| 0x00   | u8 id       | Command ID (0xBE)              |
| 0x01   | u8 sub_id   | Command sub-ID (0x02)          |
| 0x02   | u8 status   | Silent mode status (see below) |
| 0x03   | u8 unk[11]  | Unknown (zeros)                |

The silent mode status values are `0x00` for it being on and `0x08` for it being off.

### Sport mode

The watch supports a wide variety of sports, allowing the user to enter "sport mode" when performing such sports. (it is still unclear what is tracked differently in this mode)

When entering/leaving sport mode, the watch sends the following response:

| Offset | Type + name | Description                |
|--------|-------------|----------------------------|
| 0x00   | u8 id       | Command ID (0xFD)          |
| 0x01   | u8 sub_id   | Command sub-ID (see below) |
| 0x02   | u8 kind     | Sport kind (see below)     |
| 0x03   | u8 unk      | Unknown                    |

The sub-ID values are `0x11` when starting sport mode and `0x00` when exiting/finishing it.

These are the sport kinds supported by the watch:

| Value | Description         |
|-------|---------------------|
| 0x01  | Jogging             |
| 0x02  | Biking              |
| 0x08  | Climbing            |
| 0x09  | Fast running        |
| 0x0A  | Basketball          |
| 0x0B  | Football            |
| 0x12  | Spinning            |
| 0x13  | Yoga                |
| 0x15  | Indoor running      |
| 0x16  | Gimnastics          |
| 0x17  | Rowing              |
| 0x19  | Integrated training |

## Requests/responses via `CHAR_DATA2_RW` and `CHAR_DATA2_N`

The requests described in this section are all sent via `CHAR_DATA2_RW`.
The responses described in this section are all received via `CHAR_DATA2_N`.

### Pulss

Like [seen above](#pulses), a similar response gets sent by the watch in this case, the only difference being a distinct command ID of value `0x12`.

### Heart rate

The following request can be sent to retrieve all recorded heart rate data from the watch:

| Offset | Type + name | Description           |
|--------|-------------|-----------------------|
| 0x00   | u8 id       | Command ID (0x18)     |
| 0x01   | u8 sub_id   | Command sub-ID (0xFA) |

The watch will start responding by sending the following response, containing recorded heart rate values for today:

| Offset | Type + name       | Description                 |
|--------|-------------------|-----------------------------|
| 0x00   | u8 id             | Command ID (0x18)           |
| 0x01   | u8 sub_id         | Command sub-ID (0x04)       |
| 0x02   | u16 date_year_be  | Year (big-endian)           |
| 0x04   | u8 date_month     | Month                       |
| 0x05   | u8 date_day       | Day                         |
| 0x06   | u8 hour           | Hour                        |
| 0x07   | u8 min            | Minute                      |
| 0x08   | u8 max_heart_rate | Maximum recorded heart rate |
| 0x09   | u8 min_heart_rate | Minimum recorded heart rate |
| 0x0A   | u8 avg_heart_rate | Average recorded heart rate |

As the fields show, this contains current max/min/average heart rates. This response is sometimes (when exactly?) sent with command ID `0xF7` instead.

This response will also be periodically sent (about every second) while being on the heart rate menu, or alternatively (why?) it will be sent with command ID value `0x16`. When this happens (in other words, when for some unknown reason responses are sent via `CHAR_DATA2_*` instead of `CHAR_GENERAL_*` and with different command IDs) this other response will also be periodically send with said frequency:

| Offset | Type + name   | Description           |
|--------|---------------|-----------------------|
| 0x00   | u8 id         | Command ID (0x16)     |
| 0x01   | u8 sub_id     | Command sub-ID (0x11) |
| 0x02   | u8 unk        | Unknown               |
| 0x03   | u8 heart_rate | Heart rate            |

When being in said

Then it will send various responses of the following kind:

| Offset | Type + name        | Description          |
|--------|--------------------|----------------------|
| 0x00   | u8 id              | Command ID (0x18)    |
| 0x01   | u16 date_year_be   | Year (big-endian)    |
| 0x03   | u8 date_month      | Month                |
| 0x04   | u8 date_day        | Day                  |
| 0x05   | u8 hour            | Hour                 |
| 0x06   | u8 heart_rates[12] | Recorded heart rates |

These responses are only sent for even hours (0AM, 2AM, 4AM, etc.), so the 12 heart rate values are for each 10min of the 2-hour period each response covers.

Finally, the responses are concluded by a final "end" response:

| Offset | Type + name | Description           |
|--------|-------------|-----------------------|
| 0x00   | u8 id       | Command ID (0x18)     |
| 0x01   | u8 sub_id   | Command sub-ID (0xFD) |
| 0x02   | u8 unk      | Unknown               |

When moving down in the heart rate menu (hence moving to the graph plotting heart rates during the day), the following response is sent by the watch:

| Offset | Type + name | Description           |
|--------|-------------|-----------------------|
| 0x00   | u8 id       | Command ID (0x16)     |
| 0x01   | u8 sub_id   | Command sub-ID (0x11) |

No response is sent when moving back up the heart rate menu whatsoever.

When being in this menu, sometimes the following alternative responses will be periodically sent about each second (instead of the two mentioned ):



The heart rate can be disabled/enabled in the watch, which is shown as not present (like when the watch was not correctly placed and the heart rate cannot be measured).

The following request can be sent to enable/disable the heart rate:

| Offset | Type + name | Description                |
|--------|-------------|----------------------------|
| 0x00   | u8 id       | Command ID (0x18)          |
| 0x01   | u8 sub_id   | Command sub-ID (see below) |

The sub-ID values are `0x01` to enable it and `0x02` to disable it. The watch will afterwards send a response identical to this request.

The watch periodically (each 10min) sends the following response:

| Offset | Type + name      | Description           |
|--------|------------------|-----------------------|
| 0x00   | u8 id            | Command ID (0x18)     |
| 0x01   | u8 sub_id        | Command sub-ID (0x03) |
| 0x02   | u16 date_year_be | Year (big-endian)     |
| 0x04   | u8 date_month    | Month                 |
| 0x05   | u8 date_day      | Day                   |
| 0x06   | u8 hour          | Hour                  |
| 0x07   | u8 min           | Minute                |
| 0x08   | u8 heart_rate    | Current heart rate    |

This is one of the many 10-min-periodic responses sent by the watch, since various data records (steps, heart rate, etc) are recorded each 10min (10:00, 10:10, 10:20, etc). Sometimes (when?) this same response is sent with command ID value `0xF7` instead.

### Recorded steps

The same request/response mechanism for retrieving recorded steps [used above](#recorded-steps) can be used here, but with a different command ID of value `0x0A` for requests and responses.

The watch will send the following response when new steps are recorded:

| Offset | Type + name       | Description             |
|--------|-------------------|-------------------------|
| 0x00   | u8 id             | Command ID (0x09)       |
| 0x01   | u16 date_year_be  | Year (big-endian)       |
| 0x03   | u8 date_month     | Month                   |
| 0x04   | u8 date_day       | Day                     |
| 0x05   | u8 hour           | Hour                    |
| 0x06   | u8 min            | Minute                  |
| 0x07   | u8 new_step_count | New recorded step count |
| 0x08   | u8 unk[10]        | Unknown                 |

Sometimes (when exactly?) a different response is sent when new steps are recorded, this one is similar to the entries sent [as seen above](#recorded-steps) but with a different command ID of value `0xB1`.
