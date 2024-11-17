#!/bin/sh

addr=127.0.0.1
port=11158
docd=./target/doc

miniserve \
	--interfaces "${addr}" \
	--port ${port} \
	"${docd}"
