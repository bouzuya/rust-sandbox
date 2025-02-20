#!/bin/sh

# describe
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.Grpcal.CreateEvent
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.CreateEventRequest
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.CreateEventResponse
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.Grpcal.GetEvent
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.GetEventRequest
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.GetEventResponse
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.Grpcal.ListEvents
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.ListEventsRequest
grpcurl --proto ./proto/grpcal.proto localhost:3000 describe grpcal.ListEventsResponse

# list events
grpcurl --proto ./proto/grpcal.proto --plaintext localhost:3000 grpcal.Grpcal.ListEvents

# create event
json=$(grpcurl --proto ./proto/grpcal.proto --plaintext -d '{"date_time":"2020-01-02T15:16:17Z","summary":"My Event 1"}' localhost:3000 grpcal.Grpcal.CreateEvent)
echo "${json}"
id=$(echo "${json}" | jq -r '.event.id')

# get event
grpcurl --proto ./proto/grpcal.proto --plaintext -d '{"id":"'"${id}"'"}' localhost:3000 grpcal.Grpcal.GetEvent

# list events
grpcurl --proto ./proto/grpcal.proto --plaintext localhost:3000 grpcal.Grpcal.ListEvents
