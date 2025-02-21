#!/bin/sh

address='localhost:3000'
proto_dir='./proto'
proto_file='./proto/grpcal_service.proto'

# describe
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.Grpcal.CreateEvent
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.CreateEventRequest
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.CreateEventResponse
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.Grpcal.GetEvent
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.GetEventRequest
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.GetEventResponse
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.Grpcal.ListEvents
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.ListEventsRequest
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" "${address}" describe grpcal.ListEventsResponse

# list events
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" --plaintext "${address}" grpcal.Grpcal.ListEvents

# create event
json=$(grpcurl --import-path "${proto_dir}" --proto "${proto_file}" --plaintext -d '{"date_time":"2020-01-02T15:16:17Z","summary":"My Event 1"}' "${address}" grpcal.Grpcal.CreateEvent)
echo "${json}"
id=$(echo "${json}" | jq -r '.event.id')

# get event
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" --plaintext -d '{"id":"'"${id}"'"}' "${address}" grpcal.Grpcal.GetEvent

# list events
grpcurl --import-path "${proto_dir}" --proto "${proto_file}" --plaintext "${address}" grpcal.Grpcal.ListEvents
