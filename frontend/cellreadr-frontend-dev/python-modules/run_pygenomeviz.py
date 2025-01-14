import sys
import os
from pygenomeviz import GenomeViz
import random
# For converting image to base64 encoding 
from io import BytesIO
import base64, json
from PIL import Image
#
import argparse

# Target & sesRNA information necessary for generating sequence maps 
length_target = int(sys.argv[1])
str_index_begin = sys.argv[2].replace('[', '').replace(']', '')
str_index_end = sys.argv[3].replace('[', '').replace(']', '')
vec_index_begin = [int(item) for item in str_index_begin.split(',')]
vec_index_end = [int(item) for item in str_index_end.split(',')]

# Saving imaging as png 
output_dir = f"{os.getcwd()}/target/site/images/temp"
random_label = str(random.random()).replace('0.', '')
save_name = f"{output_dir}/{random_label}.png"

# Plotting settings
figure_track_height = 0.2
feature_track_ratio = 1
# plotsyle "bigarrow" centers feature on track
feature_style = "bigarrow"
feature_color = "blue"

def generate_tracks():
    # Initialize object 
    gv = GenomeViz(feature_track_ratio=feature_track_ratio, show_axis=False, fig_track_height=figure_track_height)
    gv.set_scale_xticks(ymargin=0.5)

    # Iterate over sesRNAs to map onto sequence map
    for start_index,end_index in zip(vec_index_begin, vec_index_end):
        # Generating feature corresponding to sesRNA
        generate_feature(gv, int(start_index), int(end_index))

    gv.savefig(save_name)
    sys.stdout.write(f"/images/temp/{random_label}.png")
    #os.remove(save_name)

# Test if feature can be added to existing track or if new track needs to be generated 
def test_tracks(schematic, start, end):
    # Iterate over existing tracks & check if current feature overlaps w/ last feature of a given track 
    for track in schematic.feature_tracks[1:]:
        # End index of last feature for a track
        end_last = track.segments[0]._features[-1].location.end

        # Check if overlap between current feature & last feature of track 
        if start <= end_last:
            # Create a new track only if current track is the last track 
            # i.e. Only create a new track if feature overlaps with last features of ALL existing tracks
            # To prevent creation of new track everytime 
            if track.name == schematic.feature_tracks[-1].name:
                track = schematic.add_feature_track(f"{start}", segments=length_target, labelsize=1, line_kws=dict(color="white"))
                track.add_feature(start, end, strand=-1, plotstyle=feature_style, facecolor=feature_color, arrow_shaft_ratio=1)
                # Stop iterating through tracks once feature has been added to track
                break
        else:
            # Feature added to first availible track 
            track.add_feature(start, end, strand=-1, plotstyle=feature_style, facecolor=feature_color, arrow_shaft_ratio=1)
            # Stop iterating through tracks once feature has been added to track
            break


# Add features to tracks while taking into account overlaps for overlapping ... i.e. add new track when necessary
def generate_feature(schematic, start, end):
    # condition 1: no track exists 
    if len(schematic.feature_tracks) == 0:
        # Create track of representing target 
        schematic.add_feature_track(name="Target", segments=length_target)
        # create primary track 
        track = schematic.add_feature_track("Main", segments=length_target, labelsize=1, line_kws=dict(color="white"))
        track.add_feature(start, end, strand=-1, plotstyle=feature_style, facecolor=feature_color, arrow_shaft_ratio=1)
    # condition 2: tracks exist ... 
    else:
        # Test if feature can be added to existing track or if new track needs to be generated 
        test_tracks(schematic, start, end)

if __name__ == "__main__":
    generate_tracks()