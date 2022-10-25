#!/usr/bin/env bash

SERVICES=$(echo src/service/{attach_response,follower,gateway,gps_response,local,modem_response,packet_router,poc_lora,poc_mobile,router,scan_response,state_channel,transaction}.proto)
MESSAGES=$(echo src/{blockchain_txn,entropy,data_rate,region}.proto)
SRC_FILES="$SERVICES $MESSAGES"

protoc -I=src --cpp_out=. $SRC_FILES
