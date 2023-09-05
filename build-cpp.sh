#!/usr/bin/env bash

SERVICES=$(echo src/service/{iot_config,mobile_config,downlink,multi_buy,follower,local,packet_router,poc_lora,poc_mobile,router,state_channel,transaction}.proto)
MESSAGES=$(echo src/{blockchain_txn,entropy,data_rate,region,mapper,price_report}.proto)
SRC_FILES="$SERVICES $MESSAGES"

protoc -I=src --cpp_out=. $SRC_FILES
