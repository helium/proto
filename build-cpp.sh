#!/usr/bin/env bash

SERVICES=$(echo src/service/{iot_config,downlink,follower,gateway,local,packet_router,poc_iot,poc_mobile,router,state_channel,transaction}.proto)
MESSAGES=$(echo src/{blockchain_txn,entropy,data_rate,region,mapper}.proto)
SRC_FILES="$SERVICES $MESSAGES"

protoc -I=src --cpp_out=. $SRC_FILES
