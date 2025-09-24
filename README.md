# icomical - KOMIDA to iCal

Fetches upcoming menu from University of Antwerp's student restaurant JSON API and converts it to `.ics` ([iCalendar](https://icalendar.org/)) format, to be imported into e.g., Google Calendar.

Hardcoded features:
 - fetches up to 30 days in the future
 - only creates calendar items for:
    - daily menu
    - "street food"
    - soup
 - menu items are in Dutch
 - calendar items match the opening hours of Komida (11:30 - 13:45)
 - prints iCalendar output to stdout

The output of this program (updated daily) is available as-a-service at:

https://deemz.org/public/komida.ics

