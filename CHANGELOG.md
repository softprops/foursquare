# 0.1.13

* add categories api interface

# 0.1.12

* deserialize venue canonicalUrl

# 0.1.11

* add venues recommendations interfaces
* deserialize venue time zone and attributes, present only for details requests
# 0.1.10

* deserialize explore response warning, the suggested radius is now an Option type

# 0.1.9

* user last_name is now an Option type

# 0.1.8

* add interfaces for venue hours and tips

# 0.1.7

* expand locale support to venue details method
# 0.1.6

* support [i18n](https://developer.foursquare.com/docs/api/configuration/internationalization) by adding locale to venue request options

# 0.1.5

* implement suggest completion interface
# 0.1.4

* explore Response.query is now an Option type, None in cases where you are exploring without a query

# 0.1.3

* deserialize hours
# 0.1.2

* deserialize photos and ratings

# 0.1.1

* not all venues have an address field

# 0.1.0

* initial release