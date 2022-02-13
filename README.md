# Straight-Line Massager
![Example of a Straight Line in Union Lake, New Jersey](https://i.imgur.com/2VsQcv6.png "Straight Line: Union Lake")

Straight-Line missions are a real-world Adventure similar to Geocaching. You start from an 'origin',
and then walk in a completely straight line to your destination. The adventure was conceived
and spearheaded by a Youtuber named [GeoWizard](https://www.youtube.com/c/GeoWizard).

Lines are scored based on 2 criteria:

* "Burdell Score" (from www.scoremyline.com)
* Bucket Score (Platinum, Gold, Silver, Bronze) - based on your maximum deviation from the ideal line

In order to get either score, you have to get your data into a workable format. That's where
straight-line-massager comes in.

# What the Program itself does
This will simply take a `.gpx` file and turn it into a `.csv` file of latitude/longitude pairs.

www.scoremyline.com requires a CSV in this format to tell you your Burdell Score / Max Deviation.

A `.gpx` file is a very common format produced by apps like 'Routes' on Android, Handheld GPS Units, Strava, etc.
This lets you use your normal app for the hike, and then convert it for scoring.

# Using the program
This program is really only meant to be useful to me, so there's not too many convenience
features.

In order to convert a `.gpx` file you want to follow these steps:

* Make sure you have an up-to-date Rust runtime installed
* Clone this repository
* Place a file in the root folder called `to-convert.gpx`
* Run the program
* You should now have a file called `converted.csv` that can be uploaded

There are no command-line arguments, or tests, or error handling. So far, if this program
fails to do what I need it to I've just updated it. Feel free to modify the program yourself,
or send a pull request, or anything else.