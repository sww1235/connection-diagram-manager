# Project Structure and File Formats

## Project Structure

A Connection Diagram Manager project structure should have a layout similar to
the below. The only hard requirements are having the project configuration file
in the root directory of the project. All other orginizational structure is up
to the end user.

- Project Directory (root) - user specified name
	- `cdm_project.toml`
	- src - directory
		- project-file.toml
	- lib - directory - contains library files for project
		- \<manufacturer\> - directory containing manufacturer specific part definitions
			- `library-a.toml`
			- `library-b.toml`
		- \<generic\> - directory - generic part or symbol libraries should be specified in directories organized by types.
	- out - default directory for generated output like pdf files

## File Formats

All file formats are currently based on TOML, (and inspired by
[WireViz](https://github.com/formatc1702/WireViz) and the good parts of AutoCAD
Electrical).

Files must only contain [library definitions](#library-files) or [project
design data](#project-definitions) as specified in the following sections.

> [!NOTE]
> - Placeholder text in the file format descriptions below are indicated
> with `"PLACEHOLDER"`.
> - Placeholder `Rationa64` values are indicated with `[0, 0]` which is a value
> that cannot be represented by a `Rationa64`
> - All other placeholder values are indicated with end of
> line comment when possible.

> [!IMPORTANT]
> Most values are optional in project data files. As TOML doesn't have an easy
> way of representing optional values, see the datatype definitions within the
> docs.rs page for more complete details.

Some libraries will be autopopulated with default values when the program
starts. An optional flag, provided either in the project or on the command line
will prevent this and only use the values loaded from the files specified in
the project configuration file.

On import, all project data will be validated against the library definitions.
If values are found in the project that are not present in the library, an
error log will be produced, both on the command line (if running in a terminal)
and into a log file. File parsing should not stop on errors, and should
output a complete list of errors, however this is not guaranteed. Multiple
attempts to open the project may be needed to catch all errors.

When creating project data files, they need to be valid TOML documents.
See [the TOML documentation](https://toml.io/en/v1.0.0) for more details.

Per the TOML spec, root tables do not need to be defined if not needed. They
are defined in the examples below for clarity.

> [!NOTE]
> Any number that is not specifically an integer, is implemented
> internally as a
> [Rational64](https://docs.rs/num-rational/latest/num_rational/type.Rational64.html)
> to work around precision issues with floats. You must specify both a
> numerator and denominator in the array or you will get an error. As an
> example, if you wanted to represent the number 1/3, a float of 0.3333̅33̅.
> isn't exact. With Rational types, you can specify it as exactly 1/3 and be
> satisfied. Floating point numbers are still used, especially to produce
> decimal output from Rational types but all the math internally is done with
> Rational types and just the output step is converted.

All images are specified as SVG, so drawings can scale easily.

Where a color is specified, you can choose from the following options,
or specify a custom RGB color using hexadecimal #RRGGBB syntax (not finalized yet).

If anyone has official color standards/values for these, along with the
acompanying RGB values, please submit a pull request.

#### Color Value List.

| Color Name | Abbreviation | RGB Color Code | Color Standard |
| ---------- | ------------ | -------------- | -------------- |
| Red        | RED          | #FF0000        |                |
| Orange     | ORN          | #FF5100        |                |
| Yellow     | YEL          | #FFFF00        |                |
| Green      | GRN          | #00FF00        |                |
| Blue       | BLU          | #0000FF        |                |
| Purple     | PUR          | #6700FF        |                |
| Violet     | VIO          | #EE82EE        |                |
| Pink       | PNK          | #FFE4E1        |                |
| Rose       | RSE          | #FFE4E1        |                |
| Brown      | BRN          | #8B4513        |                |
| Black      | BLK          | #000000        |                |
| White      | WHT          | #FFFFFF        |                |
| Gray       | GRY          | #808080        |                |
| Grey       | GRY          | #808080        |                |
| Slate      | SLT          | #808080        |                |
| Clear      | CLR          | #FFFFFF        |                |
| Cyan       | CYN          | #00FFFF        |                |
| Aqua       | AQA          | #00FFFF        |                |

### Application Configuration File

This file must be named `cdm_config.toml` and be in the following list of
paths. If config files are present in multiple of these locations, the lower in
the list will take precedence.

All keys are optional in the configuration file. Any that are not specified
will use the default values instead.

Unit definitions in the file can be specified either with full unit names or
abbreviations. You can run the cli binary using the `--print-units`  command line flag to get
a full list of supported units.

- `~/.config/ConnectionDiagramManager/cdm_config.toml`
- `/etc/ConnectionDiagramManager/cdm_config.toml`
- `/usr/local/etc/ConnectionDiagramManager/cdm_config.toml`
- `~/Library/Preferences/ConnectionDiagramManager/cdm_config.toml`
- `<Location of Application Binary/cdm_config.toml`

```toml
# optional
# Paths can be either relative or absolute
# If a path listed is a directory, all `.toml` files within
# will be treated as library files.
# hidden files/directories will be ignored
default_library_locations = ["PLACEHOLDER PATH STRING"]

# optional
# enable the usage of a postgres database
enable_post_gres = true # PLACEHOLDER

# optional
post_gres_dsn = "PLACEHOLDER"

# optional
default_area_unit = "PLACEHOLDER"

# optional
default_length_unit = "PLACEHOLDER"

# optional
# used for cross sectional area of wires
default_cross_section_area_unit = "PLACEHOLDER"

# optional
default_electric_potential_unit = "PLACEHOLDER"

# optional
default_temperature_interval_unit = "PLACEHOLDER"

# optional
# specify to display wire sizes in USA customary units
# (AWG/circular mils instead of mm^2 when possible
use_awg = true # PLACEHOLDER

# optional
# use feet, square feet, farenheit instead of default SI units.
# Can override specific units with the default options above.
# Will set use_awg = true, unless specifically set to false.
use_usa_customary_units = true # PLACEHOLDER

# optional
# Whether to display units with up to 3 digits before the decimal place
# and adjust the displayed unit prefix appropriately,
# or display all values in their default units only.
use_engineering_prefixes = true # PLACEHOLDER

[graphics_config]
window_height = 0 # PLACEHOLDER
window_width = 0 # PLACEHOLDER
high_dpi = true # PLACEHOLDER
```

#### Application Configuration Defaults

This shows the hard coded defaults within the application. The TOML file here
is shown as an example of how to fill out a configuration file, but these
defaults are not specified in a configuration file.


```toml
default_library_files = []
enable_post_gres = false
post_gres_dsn = ""
default_area_unit = "mm²"
default_length_unit = "mm"
default_cross_section_area_unit = "mm²"
default_electric_potential_unit = "V"
default_temperature_interval_unit = "°C"
use_awg = false
use_usa_customary_units = false
use_engineering_prefixes = true
[graphics_config]
window_height = 1024
window_width = 1024
high_dpi = true
```

### Project Definition File

The project configuration file contains options and metadata for its specific project.

See package documentation for full details on required options.

TODO: add component tag format and options, wire number format and options,
cross-reference options, styles (arrow, plc, fan-in/fan-out, wire cross, wire
tee, wire connection,

```toml
# Required
project_name = "PLACEHOLDER"

# Required
load_default_libraries = true # PLACEHOLDER

# optional
# Paths can be either relative or absolute
# If a path listed is a directory, all `.toml` files within
# it or subidirectories will be treated as library files.
# Hidden files/directories are ignored
library_paths = ["PLACEHOLDER PATH STRING"]

# optional
# If this is not defined, all TOML files in the directory that the cfg file is in,
# and sub-directories will be parsed as project files
# Paths can be either relative or absolute
# If a path listed is a directory, all `.toml` files within
# it or sub-directories will be treated as project files.
# if this is not defined, then all other TOML files found within the
# root directory or subidirectories of the project will be parsed as
# project files.
# Hidden files/directories are ignored
source_paths = ["PLACEHOLDER PATH STRING"]

# optional
# Code reference used for wire ampacity checks and conduit fill, etc.
# These are complicated enough that they are currently defined in code
# rather than a configuration file.
electrical_code_standard = "PLACEHOLDER"

# optional
# IEC project code
project_code = "PLACEHOLDER"

# optional
description = "PLACEHOLDER"
```



### Library Files

Library files contain reference data for the types of entities listed
below. The library files contain common properties of the entities, such as
model, part number, size information, etc.

All entities must have a library defintion before they can be used in a `Project`.

Library files must contain at least one of the base tables as shown in this
document. A summary of base tables is listed below:

- [Cable Types](#cable-types)
- [Connector Types](#connector-types)
- [Enclosure Types](#enclosure-types)
- [Equipment Types](#equipment-types)
- [Mounting Rail Types](#mounting-rail-types)
- [Pathway Types](#pathway-types)
- [Schematic Symbol Types](#schematic-symbol-types)
- [Term Cable Types](#term-cable-types)
- [Terminal Types](#terminal-types)
- [Terminal Strip Jumper Types](#terminal-strip-jumper-types)
- [Terminal Accessory Types](#terminal-accessory-types)
- [Terminal Strip Accessory Types](#terminal-strip-accessory-types)
- [Wire Types](#wire-types)

#### Cable Types
```toml
# Table (dictonary) of all available cable types.
# A cable is defined as one or more wires mechanically attached together,
# with optional insulation and semiconducting layers, and optional shields
# if a product has a shield or additional layers, it must be defined as a cable
# wire insulation color is defined on individual wire instance
#
# In theory, all wires could be defined as cables, with one layer of insulation
# TODO: explore this idea further

# Cable_types can be composed of cable_types.
[cable_types]

# Table (dictionary) representing one cable type
# The `"PLACEHOLDER"` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a cable_type sub-table are optional
[cable_types."PLACEHOLDER"]

# SOOW, FC, FCC, TC, MC, AC, MC, UF, PLTC, MV, etc
cable_type_code = "PLACEHOLDER"

# Outer cross sectional area of cable
cross_sect_area =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Oval, Circular, Figure8
#
# If cross section is Figure8, then only 2 cable_cores must be defined.
# any more than 2 will be reported as an error during validation
cross_section = "PLACEHOLDER"

# array of tables of outer layers of cable
# define a new array instance for each layer
# Includes insulation, semiconductor, shields, screens,
# concentric neutrals, jackets, mechanical armor
[[cable_types."PLACEHOLDER".layers]]

# counted from inside to outside of cable
layer_number = 0 # PLACEHOLDER

# - Insulation
# - Semiconductor
# - Shield
# - Screen
# - ConcentricNeutral
# - Jacket
# - Armor
# - WaterBlocking
layer_type = "PLACEHOLDER"

material = "PLACEHOLDER"

# AC electric potential rating for insulation layer
ac_electric_potential_rating =  {value = [0,0], original_unit = "PLACEHOLDER"}

# DC electric potential rating for insulation layer
dc_electric_potential_rating =  {value = [0,0], original_unit = "PLACEHOLDER"}

# temp rating for insulation layer
temperature_rating =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Other insulation properties such as
# fire spread resistance, smoke generation, etc
rating = "PLACEHOLDER"

# layer thickness
thickness = {value = [0,0], original_unit = "PLACEHOLDER"}

# color of insulation or semiconductor
color = "PLACEHOLDER"


# dictionary of wire or cable cores inside cable.
# strength members are treated as a wire
[cable_types."PLACEHOLDER".cores]

# second "PLACEHOLDER" is identifier of individual core. Must be unique per cable_type
[cable_types."PLACEHOLDER".cores."PLACEHOLDER"]

# identifier of wire/cable type that core is
# the key of this key/value pair can be either WireType or CableType
WireType = "PLACEHOLDER"

# all items here are optional
# and will use defaults or cable outer jacket/insulation color if not specified
# schematic appearance of linear items
[cable_types."PLACEHOLDER".line_style]

color = "PLACEHOLDER"

secondary_color = "PLACEHOLDER"

line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [0] # PLACEHOLDER

# optional
# Dimension subtable for each cable_type. Groups common properties
[cable_types."PLACEHOLDER".dimensions]

# height of cable
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# width of cable
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# diameter of cable
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}


# Catalog subtable for each cable_type. Groups common properties
# All fields here are optional, but highly encouraged.
[cable_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# cable type model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

#### Connector Type
```toml
# table (dictionary) of all available connector types
[connector_types]

# table (dictionary) representing one connector type
# The `"PLACEHOLDER"` is the connector type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a connector_types sub-table are optional
[connector_types."PLACEHOLDER"]

# cable, pcb through hole, pcb surface mount, panel
mount_type = "PLACEHOLDER"

# optional
# D, A, etc
# Not parsed
panel_cutout = "PLACEHOLDER"

# (male, female, rpmale, rpfemale, hermaphroditic, unknown, unspecified)
gender = "PLACEHOLDER"

# optional
# connector color
# used to label the color of a flag or tag or ring on the connector
color = "PLACEHOLDER"

# optional
# component designator
component_designator = "PLACEHOLDER"

# optional
# array of schematic symbols that can represent this connector
schematic_symbols = ["PLACEHOLDER"]

# TODO: decide if these should be filepaths or directly included SVGs
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
# if not defined, a generic diagram will be used
# this is the panel representation image
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# array of which connector types mate with this connector type
# needs to be populated with sub-table key of connectors
connector_type_mate = ["PLACEHOLDER"]

# Dimension subtable for each connector-type. Groups common properties
[connector_types."PLACEHOLDER".dimensions]

# height of connector
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# width of connector
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# depth of connector
depth = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of circular connectors
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}

# Catalog subtable for each connector-type. Groups common properties
# All fields here are optional, but highly encouraged.
[connector_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# connector model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier name
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"

# Pin Info subtables for each connector-type.
# each entry in this array describes one pin
[connector_types."PLACEHOLDER".pins]

# Table of attributes for a specific pin within
# the connector.
# "PLACEHOLDER" is the id of the pin within the connector
[connector_types."PLACEHOLDER".pins."PLACEHOLDER"]

designation = "PLACEHOLDER"

# optional
label = "PLACEHOLDER"

# optional
signal_type = "PLACEHOLDER"

# optional
color = "PLACEHOLDER"

# optional
visual_rep = "PLACEHOLDER"

# optional
# pin specific gender
gender = "PLACEHOLDER"

# optional
# pin specific rating information. Not parsed
rating = "PLACEHOLDER"
```

#### Enclosure Types
```toml
# Table (dictionary) of all available enclosure_types.
# An enclosure is a physical container or space like a
# junction box, gutter or rack.
[enclosure_types]

# Table (dictionary) representing one enclosure_type
# The `"PLACEHOLDER"` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a enclosure_type sub-table are optional
[enclosure_types."PLACEHOLDER"]

# usable internal width of enclosure
usable_width =  {value = [0,0], original_unit = "PLACEHOLDER"}

# usable internal depth of enclosure
usable_depth =  {value = [0,0], original_unit = "PLACEHOLDER"}

# usable internal height of enclosure
usable_height =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Other rating information for enclosure
rating = "PLACEHOLDER"

# optional
# if not defined, a generic drawing will be used instead
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
color = "PLACEHOLDER"

# optional
# Dimension subtable for each enclosure_type. Groups common properties
[enclosure_types."PLACEHOLDER".dimensions]

# overall height of enclosure
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# overall width of enclosure
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# overall depth of enclosure
depth =  {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of enclosure
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}

# Catalog subtable for each enclosure_type. Groups common properties
# All fields here are optional, but highly encouraged.
[enclosure_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

#### Equipment Types
```toml
# Equipment type is not an abstract type of equipment
# (like PLC, relay, circuit breaker, etc), but a manufacturer product.
# This is a major difference between this and other similar software.
# There is nothing stopping you from defining generic components,
# but you will need to swap the generic definition for a manufacturer specific
# one, once you decide on a specific part number.

# dictionary of all available equipment types
[equipment_types]

# table (dictionary) representing one equipment type
# The `"PLACEHOLDER"` is the equipment type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a equipment_types sub-table are optional
[equipment_types."PLACEHOLDER"]

# (19" rack, 23" rack, 1/2 19" rack, DIN rail,
# surface wall mount, inset wall mount, panel, custom)
mount_types = ["PLACEHOLDER"]

# optional
# (audio, video, mix, lighting, networking, patch panel, power)
category = "PLACEHOLDER"

# optional
# Equipment supertype: Relay, PLC, Motor, Relay, Circuit breaker, etc.
supertype = "PLACEHOLDER"

# optional
# component designator
component_designator = "PLACEHOLDER"


# optional
# rating of equipment. Not parsed
rating = "PLACEHOLDER"

# optional
# overall visual representation of equipment
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# array of schematic symbols that can represent this equipment
schematic_symbols = ["PLACEHOLDER"]

# Dimension subtable for each equipment_type. Groups common properties
[equipment_types."PLACEHOLDER".dimensions]

# height of equipment
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# width of equipment
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# depth of equipment
depth = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of equipment
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}


# Catalog subtable for each equipment_type. Groups common properties
# All fields here are optional, but highly encouraged.
[equipment_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# equipment type model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"

# dictionary of faces that can have connectors associated with them,
# and an associated visual representation.
# Faces can be used in place of symbols to display equipment.
[equipment_types."PLACEHOLDER".faces]

# table of attributes of each face
[equipment_types."PLACEHOLDER".faces."PLACEHOLDER"]
# TODO: use custom SVG tags to store locations of connectors instead of x/y coordinates
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER


# dictionary of connectors on equipment.
[equipment_types."PLACEHOLDER".faces."PLACEHOLDER".connectors]

# table of attributes for each connector on a face of an equipment_type
# last "PLACEHOLDER" identifier is a unique identifier for the connector on each face
[equipment_types."PLACEHOLDER".faces."PLACEHOLDER".connectors."PLACEHOLDER"]

# id of connector type
connector_type = "PLACEHOLDER"

# (input, output, power input, power output, bidirectional, passive)
direction = "PLACEHOLDER"

# location of connector from bottom left of visrep of face to right
x = 0 # PLACEHOLDER

# location of connector from bottom left of visrep of face up
y = 0 # PLACEHOLDER
```

#### Mounting Rail Types
```toml
# table of mounting rail types
# mounting rails are defined and generated parametrically
# to make usage easier.
# They can also be defined with SVG segments showing beginning and end of rail
# and the join points.
#
# mounting rail types are defined as if they are being mounted horizontally
# when placed in a project, they can be oriented in any orientation.
# any SVG files need to be designed to accomodate this layout.
[mounting_rail_types]

# table of attributes for specific mounting rail type
# origin is defined as center left of mounting rail
[mounting_rail_types."PLACEHOLDER"]

# overall height of rail
# rail center point will be at
# rail_height / 2
rail_height = {value = [0,0], original_unit = "PLACEHOLDER"}

# total height of center/recessed section of mounting rail
# centered on total height
rail_center_height = {value = [0,0], original_unit = "PLACEHOLDER"}

# does mounting rail have slots
slots = true # PLACEHOLDER

# are slots rounded or rectangular
rounded_slots = true # PLACEHOLDER

# linear distance between origin and center of first slot
# will also be used for the distance between the last slot
# and the end of the rail.
first_slot_center = {value = [0,0], original_unit = "PLACEHOLDER"}

# linear center to center distance between slots.
slot_center_to_center = {value = [0,0], original_unit = "PLACEHOLDER"}

# slot length, includes length of rounded ends
slot_length = {value = [0,0], original_unit = "PLACEHOLDER"}

slot_height = {value = [0,0], original_unit = "PLACEHOLDER"}

# the length of rail as specified by the manufacturer/supplier part number
standard_rail_length = [0,0]

# User specified minimum length.
# If not specified, will be set to 2x the first_slot_center distance
# if instance length is set smaller than default minimum_rail_length
# and no_partial_holes is false, then minimum_rail_length
# will be ignored.
minimum_rail_length = {value = [0,0], original_unit = "PLACEHOLDER"}

# extend rail so there are no partial holes
no_partial_holes = true # PLACEHOLDER

# distance between top center_line and origin
top_rail_center_height = {value = [0,0], original_unit = "PLACEHOLDER"}

# distance between bottom center_line and origin
bottom_rail_center_height = {value = [0,0], original_unit = "PLACEHOLDER"}

# distance between origin and slot vertical center
# positive above origin, negative below origin
slot_vertical_center = {value = [0,0], original_unit = "PLACEHOLDER"}

# SVG files for start, end and middle of mounting rail
# minimum rail length should be set to the length of the
# start and end SVGs to not cause graphical issues
# if minimum rail length is not set, the middle SVG
# might get cut off unexpectedly.
#
# the start, middle and end images should not have lines where they join
# so when the images are placed together, there is no overlap.

start_image = "SVG PLACEHOLDER STRING" # PLACEHOLDER

middle_image = "SVG PLACEHOLDER STRING" # PLACEHOLDER

end_image = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# Catalog subtable for each mounting_rail_type. Groups common properties
# All fields here are optional, but highly encouraged.
[mounting_rail_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

#### Pathway Types
```toml
# Table (dictonary) of all available pathway types.
# This is used for things like conduit, panduit and cable tray,
# but also includes things like J-hooks, or free-air cables.
[pathway_types]

# table (dictionary) representing one pathway type
# The `"PLACEHOLDER"` is the pathway type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a pathway_types sub-table are optional
[pathway_types."PLACEHOLDER"]

# supertype of cable pathway (conduit, cable tray, etc)
supertype = "PLACEHOLDER"

# actual size measurements. Not parsed
size = "PLACEHOLDER"

# optional
trade_size = "PLACEHOLDER"

# optional
# used to display a representation of the pathway on panel diagrams
# mainly used for things like panduit or wireway mounted to panel directly
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# Interior cross sectional area - used for conduit fill calculations
cross_sect_area =  {value = [0,0], original_unit = "PLACEHOLDER"}

# primary material of pathway
material = "PLACEHOLDER"

# primary color of pathway
color = "PLACEHOLDER"


# optional
# material properties/rating. Not parsed.
# voltage/temp/flamability/etc
rating = "PLACEHOLDER"

# optional
# Dimension subtable for each pathway_type. Groups common properties
[pathway_types."PLACEHOLDER".dimensions]

# height of equipment
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# width of equipment
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# depth of equipment
depth = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of equipment
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}

# Catalog subtable for each pathway_type. Groups common properties
# All fields here are optional, but highly encouraged.
[pathway_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# pathway type model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"

# all items here are optional
# and will use defaults if not specified
# schematic appearance of linear items
[pathway_types."PLACEHOLDER".line_style]

color = "PLACEHOLDER"

secondary_color = "PLACEHOLDER"

line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [0] # PLACEHOLDER
```

#### Schematic Symbol Types
```toml
# table of schematic symbol types
# These usually represent multiple different models/manufacturers of equipment
# but can be used to represent just 1 equipment type if desired
# symbols should be layed out for a horizontal orientation when defined.
# instances can be rotated.
[schematic_symbol_types]

# table of attributes for a specific symbol
[schematic_symbol_types."PLACEHOLDER"]

visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# Short descriptive name.
# Can contain spaces and special characters
name = "PLACEHOLDER"

# optional free-form description
description = "PLACEHOLDER"

# if this is true, svg will be searched
# for special tags that indicate where dashed link lines
# will connect.
# this is used for things like relays and contactors
supports_links = true # PLACEHOLDER
```

#### Term Cable Types
```toml
# Table (dictonary) of all available term_cable_types.
# A term_cable or Pre-terminated cable is an assembly of
# a cable_type or wire_type, and connectors. It may be manufactured or custom-assembled
# but it is used in the project as an assembled unit, rather than being
# assembled as part of the project.
# term cables can only have two ends, but each end can have
# a fan out or split with multiple connectors
[term_cable_types]

# Table (dictionary) representing one term_cable_type
# The `"PLACEHOLDER"` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a term_cable_types sub-table are optional
[term_cable_types."PLACEHOLDER"]

nominal_length =  {value = [0,0], original_unit = "PLACEHOLDER"}

# actual length of cable
length =  {value = [0,0], original_unit = "PLACEHOLDER"}

# all items here are optional
# and will use defaults or outer insulation color if not specified
# schematic appearance of linear items
[term_cable_types."PLACEHOLDER".line_style]

color = "PLACEHOLDER"

secondary_color = "PLACEHOLDER"

line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [0] # PLACEHOLDER

# Catalog subtable for each cable_type. Groups common properties
# All fields here are optional, but highly encouraged.
[term_cable_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# term_cable type model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"

# The flexible portion of the term_cable.
# The second "PLACEHOLDER" either needs to be wire_type or cable_type, to indicate if
# the included core_id is for a wire_type or cable_type
[term_cable_types."PLACEHOLDER"."PLACEHOLDER"]

# either a wire_type id or a cable_type id based on what is defined above.
core_id = "PLACEHOLDER"

# table of connectors attached to one end of term_cable
[term_cable_types."PLACEHOLDER".end1]

# table defining a specific connector on end 1
# connector id is end "PLACEHOLDER"
[term_cable_types."PLACEHOLDER".end1."PLACEHOLDER"]

# ID of connector type
connector_type = "PLACEHOLDER"

# array of tables of core to connector pin mappings for each connector
# specify one table for each pin-core mapping
[[term_cable_types."PLACEHOLDER".end1."PLACEHOLDER".terminations]]

core = "PLACEHOLDER"
pin = "PLACEHOLDER"

# table of connectors attached to the other end of term_cable
[term_cable_types."PLACEHOLDER".end2]

# table defining a specific connector on end 2
# connector id is end "PLACEHOLDER"
[term_cable_types."PLACEHOLDER".end2."PLACEHOLDER"]

# ID of connector type
connector_type = "PLACEHOLDER"

# array of tables of core to connector pin mappings for each connector
# specify one table for each pin-core mapping
[[term_cable_types."PLACEHOLDER".end2."PLACEHOLDER".terminations]]

core = "PLACEHOLDER"
pin = "PLACEHOLDER"
```

#### Terminal Types
```toml
# Table (dictionary) of all available terminal types.
# Terminals are separated out into their own category
# due to some special case things with them, including
# the accessories, and ganging.
# Terminal definitions include both DIN rail mounted terminals, WAGO lever nuts,
# Wire nuts
# Ferrules, ring/space/fork terminals, etc should be defined as connectors since
# they associate with wires
[terminal_types]

# Table (dictionary) of all attributes on one particular terminal type
[terminal_types."PLACEHOLDER"]

# optional
color = "PLACEHOLDER"

# optional
secondary_color = "PLACEHOLDER"

# optional
# used to display a representation of the terminal on panel diagrams
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# array of schematic symbols that can represent this terminal_type
schematic_symbols = ["PLACEHOLDER"]

# optional
# component designator
component_designator = "PLACEHOLDER"

# If this terminal type accepts plug in accessories
# like fuses or component holders
accepts_accessories = true # PLACEHOLDER

# Fuse terminal block
# only set to true if the terminal block is a true fuse only terminal block
# and not a terminal block that can accept a fuse holder accessory
fuse_terminal = true # PLACEHOLDER

# optional
# fuse rating inside terminal block. Not parsed.
# only use if fuse_terminal = true
fuse_rating = "PLACEHOLDER"

# optional
# if there is an indicator present
# usually this is an LED on a fuse holder or integrated into the terminal block
indicator_present = true # PLACEHOLDER

# non-parsed string indicating the indicator type
indicator_type = "PLACEHOLDER"

# non-parsed string for indicator voltage/current ratings
indicator_rating = "PLACEHOLDER"

# if there is a discrete component embedded inside the terminal
# like a Diode or resistor
# This should only be marked if the component is non-replaceable while in
# a terminal strip or not replaceable at all.
discrete_component_present = true # PLACEHOLDER

# non-parsed string for component rating
discrete_component_rating = "PLACEHOLDER"

# type of discrete component
# resistor, diode, etc.
discrete_component_type = "PLACEHOLDER"

# if there is an integrated, non-removable disconnect present
# if the disconnect is removable, use an accessory
integrated_disconnect_present = true # PLACEHOLDER

# dictionary defining terminal layers
# at least one layer is required for a terminal
# last "PLACEHOLDER" is unique layer identifier within terminal
[terminal_types."PLACEHOLDER".layers."PLACEHOLDER"]

# array defining the number of connection points per terminal layer
# define 1 instance of this table array per connection point per layer
[[terminal_types."PLACEHOLDER".layers."PLACEHOLDER".connections]]

# connection designation
# must be unique among connection points on a layer
# only used to fill out the internal_connections section below
connection_description = "PLACEHOLDER"

# connection type of terminal
# allowed options are:
# - Screw terminal
# - Bolt
# - Plug-in
# - Push-in
# - Quick Connect
# - Spade
# - Spring Cage
# - Lever Lock
connection_type = "PLACEHOLDER"

# optional
# connection entry angle
entry_angle = "PLACEHOLDER"

# maximum number of wires allowed to be connected to this terminal.
# can be lower than manufacturer recommended values
maxiumum_wires = 0 # PLACEHOLDER

maximum_wire_cross_section = {value = [0,0], original_unit = "PLACEHOLDER"}

minimum_wire_cross_section = {value = [0,0], original_unit = "PLACEHOLDER"}

# what types of wire/connectors are supported by terminal connection
# current supported list is:
# - solid
# - stranded
# - stranded_ferrule
# - spade
wire_types_accepted = ["PLACEHOLDER"]

# internal connections within terminal block
# define one instance of this table array per set of connected terminals
[[terminal_types."PLACEHOLDER".internal_connections]]

# array of terminal designations.
# use layer_designation.connection_designation in each array value
# to show what terminals are connected together
connected_connections = ["PLACEHOLDER"]

# used to indicate a connection from this set of internal connections
# to the mounting rail.
# mainly used for PE/grounding terminal blocks.
mount_connection = true # PLACEHOLDER


# optional
# Dimension subtable for each terminal_type. Groups common properties
[terminal_types."PLACEHOLDER".dimensions]

# overall height of terminal
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# overall width of terminal
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# overall depth of terminal
depth =  {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of terminal
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}

# Catalog subtable for each terminal_block_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

#### Terminal Strip Jumper Types
```toml
# Table of terminal block jumpers in library
[terminal_strip_jumper_types]

# table of attributes for one jumper type
[terminal_strip_jumper_types."PLACEHOLDER"]

# terminal block types compatible with
# If a jumper is compatible with multiple sizes of terminal blocks
# like the phoenix contact reducing bridges, then use the per-pin arrays to specify
compatible_terminal_type = ["PLACEHOLDER"]

number_of_positions = 0 # PLACEHOLDER

# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# array of schematic symbols that can represent this terminal strip jumper type
schematic_symbols = ["PLACEHOLDER"]

color = "PLACEHOLDER"

# optional
# per pin compatible terminal_block_types
# specify an array of terminal_block_types per pin
# terminal block jumpers are reversable when specified in a terminal_strip
pin_compatible_terminal_types = [["PLACEHOLDER"], ["PLACEHOLDER"]]

# optional
# Dimension subtable for each terminal_strip_jumper_type. Groups common properties
[terminal_strip_jumper_types."PLACEHOLDER".dimensions]

# overall height of terminal
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# overall width of terminal
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# overall depth of terminal
depth =  {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of terminal
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}

# Catalog subtable for each terminal_block_jumper_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_strip_jumper_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

#### Terminal Accessory Types
```toml
# Terminal accessories are items that insert into a terminal block
# like fuse holders, component holders, disconnect switches, etc
[terminal_accessory_types]

# Table of attributes for a specific terminal_accessory_type
[terminal_accessory_types."PLACEHOLDER"]

# fuse holder, component holder, disconnect_blade, etc
accessory_supertype = "PLACEHOLDER"

# array of compatible terminal_type IDs
compatible_terminal_type = ["PLACEHOLDER"]


# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# array of schematic symbols that can represent this terminal accessory
schematic_symbols = ["PLACEHOLDER"]

color = "PLACEHOLDER"

# optional
# Dimension subtable for each terminal_accessory_type. Groups common properties
[terminal_accessory_types."PLACEHOLDER".dimensions]

# overall height of terminal accessory
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# overall width of terminal accessory
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# overall depth of terminal accessory
depth =  {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# diameter of terminal accessory
diameter = {value = [0,0], original_unit = "PLACEHOLDER"}

# Catalog subtable for each terminal_accessory_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_accessory_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

#### Terminal Strip Accessory Types
```toml
# Table of terminal strip accessories in library
# Terminal strip accessories are things like end plates or spacers
# that are incorporated into a terminal_strip linearly and
# interface with terminals
# This does not include things like DIN rail stops.
[terminal_strip_accessory_types]


# table of attributes for one terminal_block_accessory_type
[terminal_strip_accessory_types."PLACEHOLDER"]

# terminal types compatible with
compatible_terminal_type = ["PLACEHOLDER"]

# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = "SVG PLACEHOLDER STRING" # PLACEHOLDER

# optional
# array of schematic symbols that can represent this terminal strip accessory type
schematic_symbols = ["PLACEHOLDER"]

color = "PLACEHOLDER"

# optional
# Dimension subtable for each terminal_accessory_type. Groups common properties
[terminal_strip_accessory_types."PLACEHOLDER".dimensions]

# overall height of terminal accessory
height = {value = [0,0], original_unit = "PLACEHOLDER"}

# overall width of terminal accessory
width = {value = [0,0], original_unit = "PLACEHOLDER"}

# optional
# overall depth of terminal accessory
depth =  {value = [0,0], original_unit = "PLACEHOLDER"}


# Catalog subtable for each terminal_strip_accessory_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_strip_accessory_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```


#### Wire Types
```toml
# Table (dictonary) of all available wire types.
# A wire is defined as a material (not necessarily conductive) with optional insulation.
# if a product has a shield or additional layers, it must be defined as a cable
# insulation color is defined on individual wire instance
[wire_types]

# Table (dictionary) representing one wire type
# The `"PLACEHOLDER"` is the wire type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a wire_types sub-table are optional
[wire_types."PLACEHOLDER"]

# THWN, XHHN, etc
wire_type_code = "PLACEHOLDER"

# copper, alumninum, ACSR, steel, glass, plastic
material = "PLACEHOLDER"

insulated = true # PLACEHOLDER

# PVC, Nylon, thermoplastic, etc
insulation_material = "PLACEHOLDER"

insulation_thickness =  {value = [0,0], original_unit = "PLACEHOLDER"}

# the cross sectional area of the conductor
conductor_cross_sect_area =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Nominal cross section of wire
nominal_cross_section =  {value = [0,0], original_unit = "PLACEHOLDER"}

# including insulation
overall_cross_sect_area =  {value = [0,0], original_unit = "PLACEHOLDER"}

# If conductor is stranded
stranded = true # PLACEHOLDER

# number of strands if cable is stranded. overriden to 1 if wire is not stranded
num_strands = 0 # PLACEHOLDER

strand_cross_sect_area = {value = [0,0], original_unit = "PLACEHOLDER"}

# AC voltage rating of insulation
ac_insulation_potential_rating =  {value = [0,0], original_unit = "PLACEHOLDER"}

# DC voltage rating of insulation
dc_insulation_potential_rating =  {value = [0,0], original_unit = "PLACEHOLDER"}

# temperature rating of insulation.
insulation_temperature_rating =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Other insulation properties such as
# flamability or smoke generation
insulation_rating = "PLACEHOLDER"

insulation_color = "PLACEHOLDER"

secondary_insulation_color = "PLACEHOLDER"

# all items here are optional
# and will use defaults or insulation color values if not specified
# schematic appearance of linear items
[wire_types."PLACEHOLDER".line_style]

color = "PLACEHOLDER"

secondary_color = "PLACEHOLDER"

line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [0] # PLACEHOLDER


# Catalog subtable for each wire_type. Groups common properties
# All fields here are optional, but highly encouraged.
[wire_types."PLACEHOLDER".catalog]

# manufacturer name
manufacturer = "PLACEHOLDER"

# wire type model description
model = "PLACEHOLDER"

# free text field for larger descriptions
description = "PLACEHOLDER"

# [internal] part number
part_number = "PLACEHOLDER"

# manufacturer part number
manufactuer_part_number = "PLACEHOLDER"

# supplier
supplier = "PLACEHOLDER"

# supplier part number
supplier_part_number = "PLACEHOLDER"
```

### Project Definitions

Project files contain definitions for each unique entity, part or component in the project and how they are connected together and located.

Projects consist of the following entities:

- [Cables](#cables)
- [Connections](#connections)
- [Connectors](#connectors)
- [Enclosures](#enclosures)
- [Equipment](#equipment)
- [Mounting Rails](#mounting-rails)
- [Pathways](#pathways)
- [Schematic Symbols](#schematic-symbols)
- [Term Cables](#term-cables)
- [Terminal Strips](#terminal-strips)
- [Wires](#wires)

#### Cables
```toml
# table of all cables within project
[cables]

# table of attributes on cable instance
[cables."PLACEHOLDER"]
# ID of cable type
cable_type = "PLACEHOLDER"

# structured name / cable number
identifier = "PLACEHOLDER"

# optional description
description = "PLACEHOLDER"

# ID of pathway instance
pathway = "PLACEHOLDER"

length =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Physical Location Information
[cables."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

[cables."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[cables."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"
```

#### Connections
```toml
# Connections between two objects, commonly either wires/cables/term_cables and a terminal/connector on equipment
# This is the only root level item in the project definition that is an array rather than a table with sub-tables
# This is because there are no human generated identifiers. Individual connections are tracked internally.

# There are no specific rules for what types end1 and end2 can be, but
# suggested that end1 be the wire/cable type and end2 be the
# terminal/equipment/connector, etc

# replace end1 and end2 with options from the following list:
# - Wire
# - Cable
# - TermCable
# - Equipment
# - TerminalStrip
# - Connector

# The "PLACEHOLDER" text should be replaced with the ID of the entity

[[connections]]

end1 = "PLACEHOLDER"

end2 = "PLACEHOLDER"
```

#### Connectors
```toml
[connectors]

[connectors."PLACEHOLDER"]
connector_type = "PLACEHOLDER"

[connectors."PLACEHOLDER".symbol_style]
color = "PLACEHOLDER"
line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}
```
#### Enclosures
```toml
# table of all enclosure instances defined in project
[enclosures]

# table of attributes on enclosure instance
[enclosures."PLACEHOLDER"]
# ID of enclosure type
enclosure_type = "PLACEHOLDER"

# structured name
identifier = "PLACEHOLDER"

# optional description
description = "PLACEHOLDER"

# Physical Location Information
[enclosures."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

[enclosures."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[enclosures."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"

# dictionary of tables of sublocations/mounting locations within the enclosure
# used to represent DIN rail, or just specific coordinate locations in a specific location

# TODO: finish this
# Individual mount_points are defined as an enum with the following options:
# - CoordinatePair => Represents a coordinate pair on the backplane
# - MountingRail => Represents a mounting rail installed on a backplane
# individual DIN rails on a backplane, and then the distance along the DIN rail
# individual keystone slots on a panel
# rack units / sub rack units within a rack
[enclosures."PLACEHOLDER".mount_points.CoordinatePair."PLACEHOLDER"]

# optional mounting rail id
# this ID must be defined in the project.
mounting_rail_id = "PLACEHOLDER"

# distance from left side of parent enclosure or location
x =  {value = [0,0], original_unit = "PLACEHOLDER"}

# distance from bottom of parent enclosure
y =  {value = [0,0], original_unit = "PLACEHOLDER"}

# distance along left side of location or rail
# allows you to not have to specify another sub-location for every single rail mounted component
distance = {value = [0,0], original_unit = "PLACEHOLDER"}
```

#### Equipment
```toml
# dictionary of equipment defined in project
[equipment]

# table of attributes for an equipment instance
[equipment."PLACEHOLDER"]

# ID of equipment type
equipment_type = "PLACEHOLDER"

# structured name
identifier = "PLACEHOLDER"

# must be in list of mounting types defined on equipment type
mounting_type = "PLACEHOLDER"

# optional
# ID of enclosure instance
enclosure = "PLACEHOLDER"

# optional
# enclosure must also be defined
# ID of mount point (within an enclosure)
mount_point = "PLACEHOLDER"

# optional description
description = "PLACEHOLDER"

# Physical Location Information
[equipment."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"


[equipment."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

[equipment."PLACEHOLDER".symbol_style]
color = "PLACEHOLDER"
line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}

# custom fields for user specified data. Not parsed
[equipment."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"
```

#### Mounting Rails
```toml
# list of mounting rails in project
[mounting_rails]

# table of attributes for a specific mounting rail
[mounting_rails."PLACEHOLDER"]

mounting_rail_type = "PLACEHOLDER"

length = {value = [0,0], original_unit = "PLACEHOLDER"}

[mounting_rails."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# Physical Location Information
[mounting_rails."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[mounting_rails."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"
```

#### Pathways
```toml
# Table of all pathways defined in project
[pathways]

# Table of attributes on a pathway instance
[pathways."PLACEHOLDER"]

# ID of pathway type
pathway_type = "PLACEHOLDER"

# structured name / pathway identifier
identifier = "PLACEHOLDER"

# optional description
description = "PLACEHOLDER"

length =  {value = [0,0], original_unit = "PLACEHOLDER"}

# Physical Location Information
[pathways."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

[pathways."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[pathways."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"
```

#### Term Cables
```toml
# table of all term_cables in project
[term_cables]

# table of attributes on a pathway instance
[term_cables."PLACEHOLDER"]

# ID of term_cable type
term_cable_type = "PLACEHOLDER"

# structured name / cable number
identifier = "PLACEHOLDER"

# optional description
description = "PLACEHOLDER"

# ID of pathway instance
pathway = "PLACEHOLDER"

# Physical Location Information
[term_cables."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

[term_cables."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[term_cables."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"
```

#### Terminal Strips
```toml
# table of all terminal strips defined in the project
# all terminal blocks are part of a terminal strip
# a terminal strip is a collection of one or more terminal blocks
[terminal_strips]

# table of attributes for a specific terminal strip
[terminal_strips."PLACEHOLDER"]

# structured name/tag strip ID / terminal strip name
identifier = "PLACEHOLDER"

# containing enclosure id
enclosure = "PLACEHOLDER"

# mounting rail id
mounting_rail = "PLACEHOLDER"

# Physical Location Information
[terminal_strips."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

[terminal_strips."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[terminal_strips."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"

# array of tables defining individual terminal blocks
# in terminal_strip.
# Definitions proceed left to right, horizontally or top to bottom vertically
[[terminal_strips."PLACEHOLDER".terminals]]

# number used for display order, defined left to right
terminal_number = 0 # PLACEHOLDER

# structured name / terminal number
identifier = "PLACEHOLDER"

# optional descriptive label
label = "PLACEHOLDER"

# terminal functional accessories
# These are things like fuses/fuse holders, component holders,
# lights, etc.
# "PLACEHOLDER" is accessory_type_id
accessories = ["PLACEHOLDER"]

[terminal_strips."PLACEHOLDER".terminals.symbol_style]
color = "PLACEHOLDER"
line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}


# TODO: this should probably be an embedded table
# defining either terminal or terminal_strip_accessory type
# must be defined under the defintion of the terminal_block array it applies to
# second "PLACEHOLDER" can either be `Terminal` or `Accessory`
[terminal_strips."PLACEHOLDER".terminals."PLACEHOLDER"]

# ID of terminal_block_type or terminal_strip_accessory_type
component_type = "PLACEHOLDER"

# array of jumpers defined in terminal strip
# these are only jumpers that exist within
# one terminal strip.
# wire jumpers that cross terminal strips
# should be defined as wires
[[terminal_strips."PLACEHOLDER".jumpers]]

# id of jumper type
jumper_type = "PLACEHOLDER"

# structured name / terminal number
identifier = "PLACEHOLDER"

# optional descriptive label
label = "PLACEHOLDER"

# array of `terminal_number`s as defined in the terminals array
# that indicate which terminals within a terminal strip
# this jumper connects
# can optionally have the terminal layer indicated with a dot and
# the terminal layer designation, allowing for multi-layer jumpers
jumper_connections = ["PLACEHOLDER"]

[terminal_strips."PLACEHOLDER".jumpers.symbol_style]
color = "PLACEHOLDER"
line_thickness = {value = [0,0], original_unit = "PLACEHOLDER"}
```

#### Wires
```toml
# dictionary of wires defined in project
# wires can only have two ends
# Wires within cables are assigned IDs automatically and are not listed here
[wires]

# table of attributes for wire instance
[wires."PLACEHOLDER"]

# ID of wire type
wire_type = "PLACEHOLDER"

# structured name / wire number
identifier = "PLACEHOLDER"

# optional description
description = "PLACEHOLDER"

# ID of containing pathway instance
pathway = "PLACEHOLDER"

# wire length
length =  {value = [0,0], original_unit = "PLACEHOLDER"}

# will be checked for 1 pin only
# intended for things like ferrules, ring terminals, etc.
end1_connector_type = "PLACEHOLDER"

end2_connector_type = "PLACEHOLDER"

# Physical Location Information
[wires."PLACEHOLDER".physical_location]

street_address = "PLACEHOLDER"
city = "PLACEHOLDER"
state = "PLACEHOLDER"
zip_code = "PLACEHOLDER"
latitude = [0,0]
longitude = [0,0]
structured_location_id = "PLACEHOLDER"
planet = "PLACEHOLDER"
building = "PLACEHOLDER"

[wires."PLACEHOLDER".iec_codes]
location = "PLACEHOLDER"
installation = "PLACEHOLDER"

# custom fields for user specified data. Not parsed
[wires."PLACEHOLDER".user_fields]
user0 = "PLACEHOLDER"
user1 = "PLACEHOLDER"
user2 = "PLACEHOLDER"
user3 = "PLACEHOLDER"
user4 = "PLACEHOLDER"
user5 = "PLACEHOLDER"
user6 = "PLACEHOLDER"
user7 = "PLACEHOLDER"
user8 = "PLACEHOLDER"
user9 = "PLACEHOLDER"
```

### SVG Files

All SVG files should be valid SVG 1.1 files.

If an SVG is representing an object with a size in the real world, the SVG
units should be in real world units, not screen units like pixels.

No custom attributes are needed for the SVGs to be displayed in the
application, but several are detailed below that add functionality to the plain
SVG files.

All custom attributes are defined as
[`data-`](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/data-*)
attributes for ease of use.

If more than one of a attribute that has its value replaced on load, exists on
an SVG tag, this is considered an error and will result in the file being
rejected by the application.

#### Common Attributes

##### Reference Designator
**Attribute Tag:** `data-ref-des`

The reference designator (referred to as a `TAG` in other CAD
software, is the main human facing label that is the main cross reference
between schematic / panel / reality

##### Manufacturer
**Attribute Tag:** `data-manufacturer`

Any `<text>` tags within the SVG with this attribute will have
their contents replaced with the `<entity_type>.catalog.manufacturer` found on the entity if
populated during rendering.

##### Model
**Attribute Tag:** `data-model`

Any `<text>` tags within the SVG with this attribute will have
their contents replaced with the `<entity_type>.catalog.model` found on the entity if
populated during rendering.

##### Description
**Attribute Tag:** `data-description`

Any `<text>` tags within the SVG with this attribute will have
their contents replaced with the `description` found on the entity if
populated during rendering.

##### Installation
**Attribute Tag:** `data-installation`

Any `<text>` tags within the SVG with this attribute will have
their contents replaced with the `iec_codes.installation`
found on the entity if populated during rendering.

##### Location
**Attribute Tag:** `data-location`

Any `<text>` tags within the SVG with this attribute will have
their contents replaced with the `iec_codes.location`
found on the entity if populated during rendering.

##### Rating
**Attribute Tag:** `data-rating`

Any `<text>` tags within the SVG with this attribute will have
their contents replaced with the `<entity_type>.rating`
found on the entity if populated during rendering.


##### Connection Point
**Attribute Tag:** `data-connection-point`

SVG elements with this attribute will be identified as a place for wires/cables
to attach to. The value of the attribute is used as the connection identifier and
should match whatever is present on the physical device.

The visual location where a wire/cable will connect to the connection point
will be the geometric center of whatever element this tag is applied to unless
the [Connection Point Type](#connection-point-type) is also present.

##### Connection Point Type
**Attribute Tag:** `data-connection-point-type`

If this attribute is present and has a value, it will affect the visual
location of where a wire/cable will connect to a connection point. This will
also affect the direction the wire/cable will approach the connection point
from.

If this attribute is present and has no value, it will generate a log message
but has no other effect.

Possible values are:

<!-- TODO: Potentially expand this list -->
- left
- right
- top
- bottom

##### Connection Point Wire Number
**Attribute Tag:** `data-connection-point-wire-number`

Any `<text>` tags within the SVG with this attribute will have their contents
replaced with the `identifier` of the wire/cable connected to the connection
point indicated by the value of this attribute. The value of this attribute
should be the value of the entity with the matching `data-connection-point`
attribute.

##### Connection Point Description
**Attribute Tag:** `data-connection-point-label`

Any `<text>` tags within the SVG with this attribute will have their contents
replaced with the `label` of the connection point indicated by the value of
this attribute. The value of this attribute should be the value of the entity
with the matching `data-connection-point` attribute.

##### Terminal Number
**Attribute Tag:** `data-terminal-number`

Any `<text>` tags within the SVG with this attribute will have their contents
replaced with the `identifier` of the `element` within a `TerminalStrip`  of
the connection point indicated by the value of this attribute. The value of
this attribute should be the value of the entity with the matching
`data-connection-point` attribute.


#### Schematic Symbol Attributes

Schematic symbols should be made out of simple lines and shapes. They should
correspond to national/international standards whenever possible.

A library file of common schematic symbols are included with the application.

Schematic symbols use the following attributes to turn plain SVGs into "smart" symbols.

No attributes are currently required to be included in a Schematic Symbol SVG
but will be used if present.

#### Terminal
**Attribute Tag:** `data-terminal`

This attribute is a marker tag that identifies a symbol as a terminal symbol.



#### Equipment

TODO: drawings

