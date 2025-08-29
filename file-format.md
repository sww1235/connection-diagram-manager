# Project Structure and File Formats

## Project Structure

A Connection Diagram Manager project structure should have a layout similar to
the below. The only hard requirements are having the project configuration file
in the root directory of the project. All other orginizational structure is up
to the end user.

- Project Directory (root) - user specified name
	- `cdm_proj.toml`
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

Files must only contain library definitions or project design data as specified
below.

Some libraries will be autopopulated with default values when the program
starts. An optional flag, provided either in the project or on the command line
will prevent this and only use the values loaded from the files specified in
the project configuration file.

On import, all project data will be validated against the library definitions.
If values are found in the project that are not present in the library, an
error log will be produced, both on the command line (if running in a terminal)
and into a log file. File parsing will not stop on the first error, and should
output a complete list of errors, however this is not guaranteed. Multiple
attempts to open the project may be needed to catch all errors.

All file format references show the data type using angle brackets, like
`<str>`. Any arrays or inline tables are indicated with the appropriate TOML
syntax below. When filling out the files, they need to be valid TOML documents.
See [the TOML documentation](https://toml.io/en/v1.0.0) for more details.

Per the TOML spec, root tables do not need to be defined if not needed. They
are defined in the examples below for clarity.

### Application Configuration File

This file must be named `cdm_config.toml` and be in the following list of
paths. If config files are present in multiple of these locations, the lower in
the list will take precedence.

All keys are optional in the configuration file. Any that are not specified
will use the default values instead.

Unit definitions in the file can be specified either with full unit names or
abbreviations. You can run the cli binary using a TODO commandline flag to get
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
default_library_locations = [<path string>]

# optional
# enable the usage of a postgres database
enable_post_gres = <bool>

# optional
post_gres_dsn = <str>

# optional
default_length_unit = <str>

# optional
default_area_unit = <str>

# optional
# used for cross sectional area of wires
default_cross_section_area_unit = <str>

# optional
default_electric_potential_unit = <str>

# optional
default_temperature_interval_unit = <str>

# optional
# specify to display wire sizes in USA customary units
# (AWG/circular mils instead of mm^2 when possible
use_awg = <bool>

# optional
# use feet, square feet, farenheit instead of default SI units.
# Can override specific units with the default options above.
# Will set use_awg = true, unless specifically set to false.
use_usa_customary_units = <bool>

# optional
# Whether to display units with up to 3 digits before the decimal place
# and adjust the displayed unit prefix appropriately,
# or display all values in their default units only.
use_engineering_prefixes = <bool>
```

#### Application Configuration Defaults

This shows the hard coded defaults within the application. The TOML file here
is shown as an example of how to fill out a configuration file, but these
defaults are not specified in a configuration file.


```toml
default_library_files = []
enable_post_gres = false
post_gres_dsn = ""
default_length_unit = "mm"
default_area_unit = "mm²"
default_cross_section_area_unit = "mm²"
default_electric_potential_unit = "V"
default_temperature_interval_unit = "°C"
use_awg = false
use_usa_customary_units = false
use_engineering_prefixes = true
```

### Project Definition File

The project configuration file contains options and metadata for its specific project.

See package documentation for full details on required options.

TODO: add component tag format and options, wire number format and options,
cross-reference options, styles (arrow, plc, fan-in/fan-out, wire cross, wire
tee, wire connection,

```toml
# Required
project_name = <str>

# optional
load_default_libraries = <bool>

# optional
# Paths can be either relative or absolute
# If a path listed is a directory, all `.toml` files within
# will be treated as library files.
library_paths = [<path string>]

# optional
# Code reference used for wire ampacity checks and conduit fill, etc.
# These are complicated enough that they are currently defined in code
# rather than a configuration file.
electrical_code_standard = <str>

# optional
# IEC project code
project_code = <str>

# optional
description = <str>
```



### Library File

Library files must contain at least one of the base tables as shown in the file
reference below.

**NOTE:** Any number that is not specifically an integer, is implemented
internally as a `[Rational64]` to work around precision issues with floats. You
must specify both a numerator and denominator in the array or you will get an
error. As an example, if you wanted to represent the number 1/3, a float of
0.333333... isn't exact. With Rational types, you can specify it as exactly 1/3
and be satisfied. Floating point numbers are still used, especially to produce
decimal output from Rational types but all the math internally is done with
Rational types and just the output step is converted.

All images are specified as SVG, so drawings can scale easily.

```toml
# table (dictionary) of all available connector types
[connector_type]

# table (dictionary) representing one connector type
# The `<str>` is the connector type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a connector_type sub-table are optional
[connector_type.<str>]
# cable, pcb through hole, pcb surface mount, panel
mount_type = <str>

# optional
# D, A, etc
# Not parsed
panel_cutout = <str>

# (male, female, rpmale, rpfemale, hermaphroditic, unknown, unspecified)
gender = <str>

# height of connector
height = [<num>, <denom>]

# unit that height is specified in
height_unit = <str>

# width of connector
width = [<num>, <denom>]

# unit that width is specified in
width_unit = <str>

# depth of connector
depth = [<num>, <denom>]

# unit that depth is specified in
depth_unit = <str>

# optional
# diameter of circular connectors
diameter = [<num>, <denom>]

# optional
# unit that diameter is specified in
diameter_unit = <str>

# optional
# array of schematic symbols that can represent this connector
# values must be the sub-table name
schematic_symbol = [<str>]

# Catalog subtable for each connector-type. Groups common properties
# All fields here are optional, but highly encouraged.
[connector_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# connector model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>

# Pinout subtable for each connector-type.
[connector_type.<str>.pinout]

# optional
# if omitted, is set to length of specified list(s)
pincount = <int>
# at least one of pins, or pinlabels must be specified
# or pincount can be used to autopopulate both.

# array of pin designations. These have to be unique.
# if omitted, is autofilled with [1, 2, ..., pincount]
pins = [<str>]

# array of pin label descriptions
# if omitted, is autofilled with blanks
pinlabels = [<str>]

# optional
# pin color marks
# list of colors to be assigned
# goes in order of pin count/pin list
# if fewer colors are specified than pins, end of list will have no colors specified
pincolors = [<str>]

# optional
# same specifications as pincolors
pin_signal_type = [<str>]

# optional
# TODO: decide if these should be filepaths or directly included SVGs
# SVG image of connector front
visual_representation = <svg>

# optional
# SVG image shows pin layout of connector with pin labels
pin_visual_representation = <svg>

# optional
# array of which connectors mate with which other connectors
# needs to be populated with sub-table key of connectors
connector_type_mate = [<str>]

# Equipment type is not an abstract type of equipment
# (like PLC, relay, circuit breaker, etc), but a manufacturer product.
# This is a major difference between this and other similar software.
# There is nothing stopping you from defining generic components,
# but you will need to swap the generic definition for a manufacturer specific
# one, once you decide on a specific part number.

# dictionary of all available equipment types
[equipment_type]

# table (dictionary) representing one equipment type
# The `<str>` is the equipment type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a equipment_type sub-table are optional
[equipment_type.<str>]

# optional
# (19" rack, 23" rack, 1/2 19" rack, DIN rail,
# surface wall mount, inset wall mount, panel, custom)
mounting_type = [<str>]

# optional
# (audio, video, mix, lighting, networking, patch panel, power)
category = <str>

# optional
# Equipment supertype: Relay, PLC, Motor, Relay, Circuit breaker, etc.
supertype = <str>

# optional
# component designator
component_designator = <str>

# optional
# overall visual representation of equipment
visual_representation = <svg>

# optional
# array of schematic symbols that can represent this equipment
# values must be the sub-table name
schematic_symbol = [<str>]

# Catalog subtable for each equipment_type. Groups common properties
# All fields here are optional, but highly encouraged.
[equipment_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# equipment type model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>

# dictionary of faces that can have connectors associated with them,
# and an associated visual representation.
# Faces can be used in place of symbols to display equipment.
[equipment_type.<str>.faces]

# table of attributes of each face
[equipment_type.<str>.faces.<str>]
# TODO: use custom SVG tags to store locations of connectors instead of x/y coordinates
visual_representation = <svg>


# dictionary of connectors on equipment.
[equipment_type.<str>.faces.<str>.connectors]

# table of attributes for each connector on a face of an equipment_type
# last <str> identifier is a unique identifier for the connector on each face
[equipment_type.<str>.faces.<str>.connectors.<str>]

# id of connector type
connector_type = <str>

# (input, output, power input, power output, bidirectional, passive)
direction = <str>

# location of connector from bottom left of visrep of face to right
x = <integer>

# location of connector from bottom left of visrep of face up
y = <integer>

# Table (dictonary) of all available pathway types.
# This is used for things like conduit, panduit and cable tray,
# but also includes things like J-hooks, or free-air cables.
[pathway_type]

# table (dictionary) representing one pathway type
# The `<str>` is the pathway type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a pathway_type sub-table are optional
[pathway_type.<str>]

# type of cable pathway (conduit, cable tray, etc)
type = <str>

# actual size measurements. Not parsed
size = <str>

# optional
trade_size = <str>

# optional
# array of schematic symbols that can represent this pathway_type
# values must be the sub-table name
schematic_symbol = [<str>]

# optional
# used to display a representation of the pathway on panel diagrams
visual_representation = <svg>

# optional
# Interior cross sectional area - used for conduit fill calculations
cross_sect_area =  [<num>,<denom>]

# optional
cross_sect_area_unit = <str>

# primary material of pathway
material = <str>


# Catalog subtable for each pathway_type. Groups common properties
# All fields here are optional, but highly encouraged.
[pathway_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# pathway type model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>

# Table (dictonary) of all available wire types.
# A wire is defined as a material (not necessarily conductive) with optional insulation.
# if a product has a shield or additional layers, it must be defined as a cable
# insulation color is defined on individual wire instance
[wire_type]

# Table (dictionary) representing one wire type
# The `<str>` is the wire type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a wire_type sub-table are optional
[wire_type.<str>]

# copper, alumninum, ACSR, steel, glass, plastic
material = <str>

insulated = <bool>

# PVC, Nylon, thermoplastic, etc
insulation_material = <str>

# THWN, XHHN, etc
wire_type_code = <str>


insulation_thickness =  [<num>,<denom>]

insulation_thickness_unit = <str>

# including insulation
overall_cross_sect_area =  [<num>,<denom>]

# the cross sectional area of the conductor
conductor_ cross_sect_area =  [<num>,<denom>]

stranded = <bool>

# number of strands if cable is stranded. overriden to 1 if wire is not stranded
num_strands = <int>

# voltage rating of insulation
insulation_potential_rating =  [<num>,<denom>]

# unit for insulation_potential
insulation_potential_rating_unit = <str>

# temperature rating of insulation.
insulation_temp_rating =  [<num>,<denom>]

insulation_temp_rating_unit = <str>

# insulation color. Pick from following options:
# GREEN, BLACK, WHITE, RED, BLUE, YELLOW, ORANGE, BROWN,
# PINK, PURPLE, SLATE, ROSE, VIOLET, AQUA, #RRGGBB (hex color code)
insulation_color = <str>

# same as insulation color
secondary_insulation_color = <str>

# Catalog subtable for each wire_type. Groups common properties
# All fields here are optional, but highly encouraged.
[wire_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# wire type model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>


# Table (dictonary) of all available cable types.
# A cable is defined as one or more wires mechanically attached together,
# with optional insulation and semiconducting layers, and optional shields
# if a product has a shield or additional layers, it must be defined as a cable
# wire insulation color is defined on individual wire instance
[cable_type]



# Table (dictionary) representing one cable type
# The `<str>` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a cable_type sub-table are optional
[cable_type.<str>]

# SOOW, FC, FCC, TC, MC, AC, MC, UF, PLTC, MV, etc
cable_type_code = <str>


# Outer cross sectional area of cable
cross_sect_area =  [<num>,<denom>]

# SOOW, FC, FCC, TC, MC, AC, MC, UF, PLTC, MV, etc
cross_section = <str>

# height of cable if oval or siamese
height =  [<num>,<denom>]

height_unit = <str>

# width of cable if oval or siamese
width =  [<num>,<denom>]

width_unit = <str>

# optional
# diameter of cable if circular
diameter =  [<num>,<denom>]

diameter_unit = <str>

# table of outer layers of cable
# Includes insulation, semiconductor, shields, screens,
# concentric neutrals, jackets, mechanical armor
[cable_type.<str>.layer]

# table of attributes for an individual layer
[cable_type.<str>.layer.<str>]

# counted from inside to outside of cable
layer_number = <int>

# insulation, semiconductor, shield, screen, concentric neutral, jacket
layer_type = <str>

material = <str>

# electric potential rating for insulation layer
electric_potential_rating =  [<num>,<denom>]

electric_potential_unit = <str>

# temp rating for insulation layer
temp_rating =  [<num>,<denom>]

temp_rating_unit = <str>

# layer thickness
thickness = [<num>, <denom>]

thickness_unit = <str>

# color of insulation or semiconductor
color = <str>


# dictionary of wire or cable cores inside cable.
# strength members are treated as a wire
[cable_type.<str>.core]

# second <str> is identifier of individual core. Must be unique per cable_type
[cable_type.<str>.core.<str>]


# third <str> should either be `wire` or `cable` to
# indicate the core is a wire or a cable
[cable_type.<str>.core.<str>.<str>]

# identifier of wire/cable type that core is
contained_type = <str>


# Catalog subtable for each cable_type. Groups common properties
# All fields here are optional, but highly encouraged.
[cable_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# cable type model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>


# Table (dictonary) of all available term_cable_types.
# A term_cable or Pre-terminated cable is an assembly of
# a cable_type or wire_type, and connectors. It may be manufactured or custom-assembled
# but it is used in the project as an assembled unit, rather than being
# assembled as part of the project.
# term cables can only have two ends, but each end can have
# a fan out or split with multiple connectors
[term_cable_type]

# Table (dictionary) representing one term_cable_type
# The `<str>` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a term_cable_type sub-table are optional
[term_cable_type.<str>]

nominal_length =  [<num>,<denom>]

nominal_length_unit = <str>

# actual length of cable
length =  [<num>,<denom>]

length_unit = <str>

# Catalog subtable for each cable_type. Groups common properties
# All fields here are optional, but highly encouraged.
[term_cable_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# term_cable type model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>

# The flexible portion of the term_cable.
# The second <str> either needs to be wire or cable, to indicate if
# the included core_id is for a wire_type or cable_type
[term_cable_type.<str>.<str>]

# either a wire_type id or a cable_type id based on what is defined above.
core_id = <str>

# table of connectors attached to one end of term_cable
[term_cable_type.<str>.end1]

# table of attributes for a specific connector on end 1
[term_cable_type.<str>.end1.<str>]

# ID of connector type
connector_type = <str>

# array of tables of core to connector pin mappings for each connector
# specify one table for each pin-core mapping
[[term_cable_type.<str>.end1.<str>.terminations]]

core = <str>
pin = <str>

# table of connectors attached to the other end of term_cable
[term_cable_type.<str>.end2]

# table of attributes for a specific connector on end 1
[term_cable_type.<str>.end2.<str>]

# ID of connector type
connector_type = <str>

# array of tables of core to connector pin mappings for each connector
# specify one table for each pin-core mapping
[[term_cable_type.<str>.end2.<str>.terminations]]

core = <str>
pin = <str>


# Table (dictonary) of all available enclosure_types.
# An enclosure is a physical container or space like a
# junction box, gutter or rack.
[enclosure_type]

# Table (dictionary) representing one enclosure_type
# The `<str>` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a enclosure_type sub-table are optional
[enclosure_type.<str>]

# optional
# array of schematic symbols that can represent this enclosure
# values must be the sub-table name
schematic_symbol = [<str>]

# overall width of enclosure
width =  [<num>,<denom>]

width_unit = <str>

# overall height of enclosure
height =  [<num>,<denom>]

height_unit = <str>

# overall depth of enclosure
depth =  [<num>,<denom>]

depth_unit = <str>

# usable internal width of enclosure
usable_width =  [<num>,<denom>]

useable_width_unit = <str>

# usable internal depth of enclosure
usable_depth =  [<num>,<denom>]

usable_depth_unit = <str>

# usable internal height of enclosure
usable_height =  [<num>,<denom>]

usable_height_unit = <str>

# Catalog subtable for each enclosure_type. Groups common properties
# All fields here are optional, but highly encouraged.
[enclosure_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# model description
model = <str>

# free text field for larger descriptions
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>


# initial value list.
#- Red (RED)
#- Orange (ORN)
#- Yellow (YEL)
#- Green (GRN)
#- Blue (BLU)
#- Purple (PUR)
#- Brown (BRN)
#- Black (BLK)
#- Gray (GRY)
#- Slate (SLT)
#- Clear (CLR)
#- Cyan (CYN)


colors = # dictionary of colors. The color name (key) must be unique.
	<str> = <str>					# name: abbreviation

TODO: Schematic symbols

TODO: Terminal definitions - also reuse for pins

TODO: Mounting rail
```

### Project Definitions

```toml
# dictionary of equipment defined in project
[equipment]

# table of attributes for an equipment instance
[equipment.<str>]

# ID of equipment type
equipment_type = <str>

# structured name
identifier = <str>

# must be in list of mounting types defined on equipment type
mounting_type = <str>

# ID of enclosure instance
enclosure = <str>

# optional description
description = <str>

[equipment.<str>.iec_codes]
location = <str>
installation = <str>


# dictionary of wires defined in project
# wires can only have two ends
# Wires within cables are assigned IDs automatically and are not listed here
[wires]

# table of attributes for wire instance
[wires.<str>]

# ID of wire type
wire_type = <str>

# structured name
identifier = <str>

# optional description
description = <str>

# ID of containing pathway instance
pathway = <str>

# wire length
length =  [<num>,<denom>]

length_unit = <str>

# will be checked for 1 pin only
# intended for things like ferrules, ring terminals, etc.
end1_connector_type = <str>

end2_connector_type = <str>

[wires.<str>.iec_codes]
location = <str>
installation = <str>

# table of all cables within project
[cables]

# table of attributes on cable instance
[cables.<str>]
# ID of cable type
cable_type = <str>

# structured name
identifier = <str>

# optional description
description = <str>

# ID of pathway instance
pathway = <str>

length =  [<num>,<denom>]

length_unit = <str>

[cables.<str>.iec_codes]
location = <str>
installation = <str>


# table of all term_cables in project
[term_cables]

# table of attributes on a pathway instance
[term_cables.<str>]

# ID of term_cable type
term_cable_type = <str>

# structured name
identifier = <str>

# optional description
description = <str>

# ID of pathway instance
pathway = <str>

[term_cables.<str>.iec_codes]
location = <str>
installation = <str>


# Table of all pathways defined in project
[pathways]

# Table of attributes on a pathway instance
[pathways.<str>]

# ID of pathway type
pathway_type = <str>

# structured name
identifier = <str>

# optional description
description = <str>

length =  [<num>,<denom>]

length_unit = <str>

[pathways.<str>.iec_codes]
location = <str>
installation = <str>

# table of all enclosure instances defined in project
[enclosures]

# table of attributes on enclosure instance
[enclosures.<str>]
# ID of enclosure type
enclosure_type = <str>

# structured name
identifier = <str>

# optional description
description = <str>

# street address, coordinates, description
phyiscal_location = <str>

[enclosures.<str>.iec_codes]
location = <str>
installation = <str>


# array of tables of sublocations/mounting locations within the enclosure
# used to represent DIN rail, or just specific coordinate locations in a specific location

# examples of subenclosures would be coordinate pairs on a backplane,
# individual DIN rails on a backplane, and then the distance along the DIN rail
# individual keystone slots on a panel
# rack units / sub rack units within a rack
# TODO: flesh this out more
[[enclosures.<str>.location]]

			x =  [<num>,<denom>]				# distance from left side of parent enclosure, specified in mm
			y =  [<num>,<denom>]				# distance from bottom of parent enclosure, specified in mm
			z =  [<num>,<denom>]				# distance from back of parent enclosure, specified in mm



TODO: Terminals, terminal blocks, jumpers,


connection =		# list of all connections defined in project, with submappings to identify the objects that are connected
				# connections are uniquely identified by concatenating the two ids of the connected objects together
				#
	- end1 = <str>					# unique identifier of connected object.
									# If connected object contains subobjects, and they are not specifically
									# connected together, but their parents are, application logic will assume
									# connection patterns for the subobjects.
	- end2 = <str>					# unique identifier of connected object. Cannot be the same as end1.

```


