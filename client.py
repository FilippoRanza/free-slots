#! /usr/bin/python

import json
import requests


def input_day_calendar():
    output = {}
    output["not_before"] = input("Insert Early Time Limit: ")
    output["not_after"] = input("Insert Late Time Limit: ")

    events = []
    while input("Add another event?[Y/n]: ") != "n":
        begin = input("Event Begin: ")
        end = input("Event End: ")
        events.append((begin, end))

    output["events"] = events
    return output


def collect_calendars():
    output = []
    while input("Insert another calendar?[Y/n]: ") != "n":
        tmp = input_day_calendar()
        output.append(tmp)
    return output

def find_free_slots(calendars):
    json_cals = json.dumps(calendars)
    resp = requests.post("http://localhost:8088/api", data=json_cals)
    if resp.status_code != 200:
        raise ValueError

    json_resp = resp.text
    return json.loads(json_resp)

def show_free_slots(free_cal):
    free_slots = free_cal['events']
    if len(free_slots) == 0:
        print("No free slots found")
    else:
        print("Found free slots")
        for b, e in free_slots:
            print(f"{b} - {e}")

def main():
    cals = collect_calendars()
    free = find_free_slots(cals)
    show_free_slots(free)


if __name__ == '__main__':
    main()

