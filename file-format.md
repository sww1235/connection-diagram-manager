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

Files must only contain library definitions or project design data as specified
below.

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

All file format references show the data type using angle brackets, like
`<str>`. Any arrays or inline tables are indicated with the appropriate TOML
syntax below. When filling out the files, they need to be valid TOML documents.
See [the TOML documentation](https://toml.io/en/v1.0.0) for more details.

Per the TOML spec, root tables do not need to be defined if not needed. They
are defined in the examples below for clarity.

**NOTE:** Any number that is not specifically an integer, is implemented
internally as a
[Rational64](https://docs.rs/num-rational/latest/num_rational/type.Rational64.html)
to work around precision issues with floats. You must specify both a numerator
and denominator in the array or you will get an error. As an example, if you
wanted to represent the number 1/3, a float of 0.3333̅33̅. isn't exact. With
Rational types, you can specify it as exactly 1/3 and be satisfied. Floating
point numbers are still used, especially to produce decimal output from
Rational types but all the math internally is done with Rational types and just
the output step is converted.

All images are specified as SVG, so drawings can scale easily.

Where a color `<str>` is specified, you can choose from the following options,
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
# hidden files/directories will be ignored
default_library_locations = [<path string>]

# optional
# enable the usage of a postgres database
enable_post_gres = <bool>

# optional
post_gres_dsn = <str>

# optional
default_area_unit = <str>

# optional
default_length_unit = <str>

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
default_area_unit = "mm²"
default_length_unit = "mm"
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

# Required
load_default_libraries = <bool>

# optional
# Paths can be either relative or absolute
# If a path listed is a directory, all `.toml` files within
# it or subidirectories will be treated as library files.
# Hidden files/directories are ignored
library_paths = [<path string>]

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
source_paths = [<path string>]

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

#### Connector Type
```toml
# table (dictionary) of all available connector types
[connector_types]

# table (dictionary) representing one connector type
# The `<str>` is the connector type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a connector_types sub-table are optional
[connector_types.<str>]

# cable, pcb through hole, pcb surface mount, panel
mount_type = <str>

# optional
# D, A, etc
# Not parsed
panel_cutout = <str>

# (male, female, rpmale, rpfemale, hermaphroditic, unknown, unspecified)
gender = <str>

# optional
# connector color
# used to label the color of a flag or tag or ring on the connector
color = <str>

# optional
# component designator
component_designator = <str>

# optional
# array of schematic symbols that can represent this connector
# values must be the sub-table name
schematic_symbol = [<str>]

# TODO: decide if these should be filepaths or directly included SVGs
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
# if not defined, a generic diagram will be used
# this is the panel representation image
visual_representation = <svg>

# optional
# array of which connector types mate with this connector type
# needs to be populated with sub-table key of connectors
connector_type_mate = [<str>]

# Dimension subtable for each connector-type. Groups common properties
[connector_types.<str>.dimensions]

# height of connector
height = {value = [<num>, <denom>], original_unit = <str>}

# width of connector
width = {value = [<num>, <denom>], original_unit = <str>}

# depth of connector
depth = {value = [<num>, <denom>], original_unit = <str>}

# optional
# diameter of circular connectors
diameter = {value = [<num>, <denom>], original_unit = <str>}

# Catalog subtable for each connector-type. Groups common properties
# All fields here are optional, but highly encouraged.
[connector_types.<str>.catalog]

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

# supplier name
supplier = <str>

# supplier part number
supplier_part_number = <str>

# Pin Info subtables for each connector-type.
# each entry in this array describes one pin
[connector_types.<str>.pins]

# Table of attributes for a specific pin within
# the connector.
# <str> is the id of the pin within the connector
[connector_types.<str>.pins.<str>]

designation = <str>

# optional
label = <str>

# optional
signal_type = <str>

# optional
color = <str>

# optional
visual_rep = <str>

# optional
# pin specific gender
gender = <str>

# optional
# pin specific rating information. Not parsed
rating = <str>
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
# The `<str>` is the equipment type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a equipment_types sub-table are optional
[equipment_types.<str>]

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
# rating of equipment. Not parsed
rating = <str>

# optional
# overall visual representation of equipment
visual_representation = <svg>

# optional
# array of schematic symbols that can represent this equipment
# values must be the sub-table name
schematic_symbols = [<str>]

# Dimension subtable for each equipment_type. Groups common properties
[equipment_types.<str>.dimensions]

# height of equipment
height = {value = [<num>, <denom>], original_unit = <str>}

# width of equipment
width = {value = [<num>, <denom>], original_unit = <str>}

# optional
# depth of equipment
depth = {value = [<num>, <denom>], original_unit = <str>}

# optional
# diameter of equipment
diameter = {value = [<num>, <denom>], original_unit = <str>}


# Catalog subtable for each equipment_type. Groups common properties
# All fields here are optional, but highly encouraged.
[equipment_types.<str>.catalog]

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
[equipment_types.<str>.faces]

# table of attributes of each face
[equipment_types.<str>.faces.<str>]
# TODO: use custom SVG tags to store locations of connectors instead of x/y coordinates
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = <svg>


# dictionary of connectors on equipment.
[equipment_types.<str>.faces.<str>.connectors]

# table of attributes for each connector on a face of an equipment_type
# last <str> identifier is a unique identifier for the connector on each face
[equipment_types.<str>.faces.<str>.connectors.<str>]

# id of connector type
connector_type = <str>

# (input, output, power input, power output, bidirectional, passive)
direction = <str>

# location of connector from bottom left of visrep of face to right
x = <integer>

# location of connector from bottom left of visrep of face up
y = <integer>
```


#### Pathway Types
```toml
# Table (dictonary) of all available pathway types.
# This is used for things like conduit, panduit and cable tray,
# but also includes things like J-hooks, or free-air cables.
[pathway_types]

# table (dictionary) representing one pathway type
# The `<str>` is the pathway type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a pathway_types sub-table are optional
[pathway_types.<str>]

# supertype of cable pathway (conduit, cable tray, etc)
supertype = <str>

# actual size measurements. Not parsed
size = <str>

# optional
trade_size = <str>

# optional
# used to display a representation of the pathway on panel diagrams
# mainly used for things like panduit or wireway mounted to panel directly
visual_representation = <svg>

# optional
# Interior cross sectional area - used for conduit fill calculations
cross_sect_area =  {value = [<num>,<denom>], original_unit = <str>}

# primary material of pathway
material = <str>

# primary color of pathway
color = <str>


# optional
# material properties/rating. Not parsed.
# voltage/temp/flamability/etc
rating = <str>

# optional
# Dimension subtable for each pathway_type. Groups common properties
[pathway_types.<str>.dimensions]

# height of equipment
height = {value = [<num>, <denom>], original_unit = <str>}

# width of equipment
width = {value = [<num>, <denom>], original_unit = <str>}

# optional
# depth of equipment
depth = {value = [<num>, <denom>], original_unit = <str>}

# optional
# diameter of equipment
diameter = {value = [<num>, <denom>], original_unit = <str>}

# Catalog subtable for each pathway_type. Groups common properties
# All fields here are optional, but highly encouraged.
[pathway_types.<str>.catalog]

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

# all items here are optional
# and will use defaults if not specified
# schematic appearance of linear items
[pathway_types.<str>.line_style]

color = <str>

secondary_color = <str>

line_thickness = {value = [<num>,<denom>], original_unit = <str>}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [<int>]
```


#### Wire Types
```toml
# Table (dictonary) of all available wire types.
# A wire is defined as a material (not necessarily conductive) with optional insulation.
# if a product has a shield or additional layers, it must be defined as a cable
# insulation color is defined on individual wire instance
[wire_types]

# Table (dictionary) representing one wire type
# The `<str>` is the wire type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a wire_types sub-table are optional
[wire_types.<str>]

# THWN, XHHN, etc
wire_type_code = <str>

# copper, alumninum, ACSR, steel, glass, plastic
material = <str>

insulated = <bool>

# PVC, Nylon, thermoplastic, etc
insulation_material = <str>

insulation_thickness =  {value = [<num>,<denom>], original_unit = <str>}

# the cross sectional area of the conductor
conductor_cross_sect_area =  {value = [<num>,<denom>], original_unit = <str>}

# including insulation
overall_cross_sect_area =  {value = [<num>,<denom>], original_unit = <str>}

# If conductor is stranded
stranded = <bool>

# number of strands if cable is stranded. overriden to 1 if wire is not stranded
num_strands = <int>

strand_cross_sect_area = {value = [<num>,<denom>], original_unit = <str>}

# AC voltage rating of insulation
ac_insulation_potential_rating =  {value = [<num>,<denom>], original_unit = <str>}

# DC voltage rating of insulation
dc_insulation_potential_rating =  {value = [<num>,<denom>], original_unit = <str>}

# temperature rating of insulation.
insulation_temperature_rating =  {value = [<num>,<denom>], original_unit = <str>}

# Other insulation properties such as
# flamability or smoke generation
insulation_rating: <str>

insulation_color = <str>

secondary_insulation_color = <str>

# all items here are optional
# and will use defaults or insulation color values if not specified
# schematic appearance of linear items
[wire_types.<str>.line_style]

color = <str>

secondary_color = <str>

line_thickness = {value = [<num>,<denom>], original_unit = <str>}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [<int>]


# Catalog subtable for each wire_type. Groups common properties
# All fields here are optional, but highly encouraged.
[wire_types.<str>.catalog]

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
```

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
# The `<str>` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a cable_type sub-table are optional
[cable_types.<str>]

# SOOW, FC, FCC, TC, MC, AC, MC, UF, PLTC, MV, etc
cable_type_code = <str>

# Outer cross sectional area of cable
cross_sect_area =  {value = [<num>,<denom>], original_unit = <str>}

# Oval, Circular, Siamese
cross_section = <str>

# array of tables of outer layers of cable
# define a new array instance for each layer
# Includes insulation, semiconductor, shields, screens,
# concentric neutrals, jackets, mechanical armor
[[cable_types.<str>.layers]]

# counted from inside to outside of cable
layer_number = <int>

# insulation, semiconductor, shield, screen, concentric neutral, jacket, armor
layer_type = <str>

material = <str>

# AC electric potential rating for insulation layer
ac_electric_potential_rating =  {value = [<num>,<denom>], original_unit = <str>}

# DC electric potential rating for insulation layer
dc_electric_potential_rating =  {value = [<num>,<denom>], original_unit = <str>}

# temp rating for insulation layer
temperature_rating =  {value = [<num>,<denom>], original_unit = <str>}

# Other insulation properties such as
# fire spread resistance, smoke generation, etc
rating = <str>

# layer thickness
thickness = {value = [<num>,<denom>], original_unit = <str>}

# color of insulation or semiconductor
color = <str>


# dictionary of wire or cable cores inside cable.
# strength members are treated as a wire
[cable_types.<str>.cores]

# second <str> is identifier of individual core. Must be unique per cable_type
[cable_types.<str>.cores.<str>]

# identifier of wire/cable type that core is
# <contained_type> can be either WireType or CableType
<contained_type> = <str>

# all items here are optional
# and will use defaults or cable outer jacket/insulation color if not specified
# schematic appearance of linear items
[cable_types.<str>.line_style]

color = <str>

secondary_color = <str>

line_thickness = {value = [<num>,<denom>], original_unit = <str>}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [<int>]

# optional
# Dimension subtable for each cable_type. Groups common properties
[cable_types.<str>.dimensions]

# height of cable
height = {value = [<num>,<denom>], original_unit = <str>}

# width of cable
width = {value = [<num>,<denom>], original_unit = <str>}

# diameter of cable
diameter = {value = [<num>,<denom>], original_unit = <str>}


# Catalog subtable for each cable_type. Groups common properties
# All fields here are optional, but highly encouraged.
[cable_types.<str>.catalog]

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
# The `<str>` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a term_cable_types sub-table are optional
[term_cable_types.<str>]

nominal_length =  {value = [<num>,<denom>], original_unit = <str>}

# actual length of cable
length =  {value = [<num>,<denom>], original_unit = <str>}

# all items here are optional
# and will use defaults or outer insulation color if not specified
# schematic appearance of linear items
[term_cable_types.<str>.line_style]

color = <str>

secondary_color = <str>

line_thickness = {value = [<num>,<denom>], original_unit = <str>}

# array of lengths/percentages of dashes and gaps
# uses same specification as SVG stroke-dasharray field.
line_appearance = [<int>]

# Catalog subtable for each cable_type. Groups common properties
# All fields here are optional, but highly encouraged.
[term_cable_types.<str>.catalog]

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
# The second <str> either needs to be wire_type or cable_type, to indicate if
# the included core_id is for a wire_type or cable_type
[term_cable_types.<str>.<str>]

# either a wire_type id or a cable_type id based on what is defined above.
core_id = <str>

# table of connectors attached to one end of term_cable
[term_cable_types.<str>.end1]

# table defining a specific connector on end 1
# connector id is end <str>
[term_cable_types.<str>.end1.<str>]

# ID of connector type
connector_type = <str>

# array of tables of core to connector pin mappings for each connector
# specify one table for each pin-core mapping
[[term_cable_types.<str>.end1.<str>.terminations]]

core = <str>
pin = <str>

# table of connectors attached to the other end of term_cable
[term_cable_types.<str>.end2]

# table defining a specific connector on end 2
# connector id is end <str>
[term_cable_types.<str>.end2.<str>]

# ID of connector type
connector_type = <str>

# array of tables of core to connector pin mappings for each connector
# specify one table for each pin-core mapping
[[term_cable_types.<str>.end2.<str>.terminations]]

core = <str>
pin = <str>
```

#### Enclosure Types
```toml
# Table (dictionary) of all available enclosure_types.
# An enclosure is a physical container or space like a
# junction box, gutter or rack.
[enclosure_types]

# Table (dictionary) representing one enclosure_type
# The `<str>` is the cable type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a enclosure_type sub-table are optional
[enclosure_types.<str>]

# usable internal width of enclosure
usable_width =  {value = [<num>,<denom>], original_unit = <str>}

# usable internal depth of enclosure
usable_depth =  {value = [<num>,<denom>], original_unit = <str>}

# usable internal height of enclosure
usable_height =  {value = [<num>,<denom>], original_unit = <str>}

# Other rating information for enclosure
rating = <str>

# optional
# if not defined, a generic drawing will be used instead
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = <svg>

# optional
color = <str>

# optional
# Dimension subtable for each enclosure_type. Groups common properties
[enclosure_types.<str>.dimensions]

# overall height of enclosure
height = {value = [<num>,<denom>], original_unit = <str>}

# overall width of enclosure
width = {value = [<num>,<denom>], original_unit = <str>}

# optional
# overall depth of enclosure
depth =  {value = [<num>,<denom>], original_unit = <str>}

# optional
# diameter of enclosure
diameter = {value = [<num>,<denom>], original_unit = <str>}

# Catalog subtable for each enclosure_type. Groups common properties
# All fields here are optional, but highly encouraged.
[enclosure_types.<str>.catalog]

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
[terminal_types.<str>]

# optional
color = <str>

# optional
secondary_color = <str>

# optional
# used to display a representation of the terminal on panel diagrams
# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = <svg>

# optional
# component designator
component_designator = <str>

# If this terminal type accepts plug in accessories
# like fuses or component holders
accepts_accessories = <bool>

# Fuse terminal block
# only set to true if the terminal block is a true fuse only terminal block
# and not a terminal block that can accept a fuse holder accessory
fuse_terminal = <bool>

# optional
# fuse rating inside terminal block. Not parsed.
# only use if fuse_terminal = true
fuse_rating = <str>

# optional
# if there is an indicator present
# usually this is an LED on a fuse holder or integrated into the terminal block
indicator_present = <bool>

# non-parsed string indicating the indicator type
indicator_type = <str>

# non-parsed string for indicator voltage/current ratings
indicator_rating = <str>

# if there is a discrete component embedded inside the terminal
# like a Diode or resistor
# This should only be marked if the component is non-replaceable while in
# a terminal strip or not replaceable at all.
discrete_component_present = <bool>

# non-parsed string for component rating
discrete_component_rating = <str>

# type of discrete component
# resistor, diode, etc.
discrete_component_type = <str>

# if there is an integrated, non-removable disconnect present
# if the disconnect is removable, use an accessory
integrated_disconnect_present = <bool>

# dictionary defining terminal layers
# at least one layer is required for a terminal
# last <str> is unique layer identifier within terminal
[terminal_types.<str>.layers.<str>]

# array defining the number of connection points per terminal layer
# define 1 instance of this table array per connection point per layer
[[terminal_types.<str>.layers.<str>.connections]]

# connection designation
# must be unique among connection points on a layer
# only used to fill out the internal_connections section below
connection_description = <str>

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
connection_type = <str>

# optional
# connection entry angle
entry_angle = <str>

# maximum number of wires allowed to be connected to this terminal.
# can be lower than manufacturer recommended values
maxiumum_wires = <int>

maximum_wire_cross_section = {value = [<num>,<denom>], original_unit = <str>}

minimum_wire_cross_section = {value = [<num>,<denom>], original_unit = <str>}

# what types of wire/connectors are supported by terminal connection
# current supported list is:
# - solid
# - stranded
# - stranded_ferrule
# - spade
wire_types_accepted = [<str>]

# internal connections within terminal block
# define one instance of this table array per set of connected terminals
[[terminal_types.<str>.internal_connections]]

# array of terminal designations.
# use layer_designation.connection_designation in each array value
# to show what terminals are connected together
connected_connections = [<str>]

# used to indicate a connection from this set of internal connections
# to the mounting rail.
# mainly used for PE/grounding terminal blocks.
mount_connection = <bool>


# optional
# Dimension subtable for each terminal_type. Groups common properties
[terminal_types.<str>.dimensions]

# overall height of terminal
height = {value = [<num>,<denom>], original_unit = <str>}

# overall width of terminal
width = {value = [<num>,<denom>], original_unit = <str>}

# optional
# overall depth of terminal
depth =  {value = [<num>,<denom>], original_unit = <str>}

# optional
# diameter of terminal
diameter = {value = [<num>,<denom>], original_unit = <str>}

# Catalog subtable for each terminal_block_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_types.<str>.catalog]

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
```

#### Terminal Block Jumper Types
```toml
# Table of terminal block jumpers in library
[terminal_strip_jumper_types]

# table of attributes for one jumper type
[terminal_strip_jumper_types.<str>]

# terminal block types compatible with
# If a jumper is compatible with multiple sizes of terminal blocks
# like the phoenix contact reducing bridges, then use the per-pin arrays to specify
compatible_terminal_type = [<str>]

number_of_positions = <int>

# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = <svg>

color = <str>

# optional
# per pin compatible terminal_block_types
# specify an array of terminal_block_types per pin
# terminal block jumpers are reversable when specified in a terminal_strip
pin_compatible_terminal_types = [[<str>], [<str>]]

# optional
# Dimension subtable for each terminal_strip_jumper_type. Groups common properties
[terminal_strip_jumper_types.<str>.dimensions]

# overall height of terminal
height = {value = [<num>,<denom>], original_unit = <str>}

# overall width of terminal
width = {value = [<num>,<denom>], original_unit = <str>}

# optional
# overall depth of terminal
depth =  {value = [<num>,<denom>], original_unit = <str>}

# optional
# diameter of terminal
diameter = {value = [<num>,<denom>], original_unit = <str>}

# Catalog subtable for each terminal_block_jumper_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_strip_jumper_types.<str>.catalog]

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
```

#### Terminal Accessory Types
```toml
# Terminal accessories are items that insert into a terminal block
# like fuse holders, component holders, disconnect switches, etc
[terminal_accessory_types]

# Table of attributes for a specific terminal_accessory_type
[terminal_accessory_types.<str>]

# fuse holder, component holder, disconnect_blade, etc
accessory_supertype = <str>

# array of compatible terminal_type IDs
compatible_terminal_type = [<str>]


# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = <svg>

color = <str>

# optional
# Dimension subtable for each terminal_accessory_type. Groups common properties
[terminal_accessory_types.<str>.dimensions]

# overall height of terminal accessory
height = [<num>, <denom>]{value = [<num>,<denom>], original_unit = <str>}

# overall width of terminal accessory
width = {value = [<num>,<denom>], original_unit = <str>}

# optional
# overall depth of terminal accessory
depth =  {value = [<num>,<denom>], original_unit = <str>}

# optional
# diameter of terminal accessory
diameter = {value = [<num>,<denom>], original_unit = <str>}

# Catalog subtable for each terminal_accessory_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_accessory_types.<str>.catalog]

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
```

#### Terminal Strip Accessories
```toml
# Table of terminal strip accessories in library
# Terminal strip accessories are things like end plates or spacers
# that are incorporated into a terminal_strip linearly and
# interface with terminals
# This does not include things like DIN rail stops.
[terminal_strip_accessory_types]


# table of attributes for one terminal_block_accessory_type
[terminal_strip_accessory_types.<str>]

# terminal types compatible with
compatible_terminal_type = [<str>]

# SVGs should be layed out for a horizontal orientation when defined.
# instances can be rotated when defined in project.
visual_representation = <svg>

color = <str>

# optional
# Dimension subtable for each terminal_accessory_type. Groups common properties
[terminal_strip_accessory_types.<str>.dimensions]

# overall height of terminal accessory
height = {value = [<num>,<denom>], original_unit = <str>}<num>, <denom>]

# overall width of terminal accessory
width = {value = [<num>,<denom>], original_unit = <str>}

# optional
# overall depth of terminal accessory
depth =  {value = [<num>,<denom>], original_unit = <str>}


# Catalog subtable for each terminal_strip_accessory_type. Groups common properties
# All fields here are optional, but highly encouraged.
[terminal_strip_accessory_types.<str>.catalog]

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
[schematic_symbol_types.<str>]

visual_representation = <svg>

# Short descriptive name.
# Can contain spaces and special characters
name = <str>

# optional free-form description
description = <str>

# if this is true, svg will be searched
# for special tags that indicate where dashed link lines
# will connect.
# this is used for things like relays and contactors
# supports_links = <bool>
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
[mounting_rail_types.<str>]

# overall height of rail
# rail center point will be at
# rail_height / 2
rail_height = {value = [<num>,<denom>], original_unit = <str>}

# total height of center/recessed section of mounting rail
# centered on total height
rail_center_height = {value = [<num>,<denom>], original_unit = <str>}

# does mounting rail have slots
slots = <bool>

# are slots rounded or rectangular
rounded_slots = <bool>

# linear distance between origin and center of first slot
# will also be used for the distance between the last slot
# and the end of the rail.
first_slot_center = {value = [<num>,<denom>], original_unit = <str>}

# linear center to center distance between slots.
slot_center_to_center = {value = [<num>,<denom>], original_unit = <str>}

# slot length, includes length of rounded ends
slot_length = {value = [<num>,<denom>], original_unit = <str>}

slot_height = {value = [<num>,<denom>], original_unit = <str>}

# the length of rail as specified by the manufacturer/supplier part number
standard_rail_length = [<num>,<denom>]

# User specified minimum length.
# If not specified, will be set to 2x the first_slot_center distance
# if instance length is set smaller than default minimum_rail_length
# and no_partial_holes is false, then minimum_rail_length
# will be ignored.
minimum_rail_length = {value = [<num>,<denom>], original_unit = <str>}

# extend rail so there are no partial holes
no_partial_holes = <bool>

# distance between top center_line and origin
top_rail_center_height = {value = [<num>,<denom>], original_unit = <str>}

# distance between bottom center_line and origin
bottom_rail_center_height = {value = [<num>,<denom>], original_unit = <str>}

# distance between origin and slot vertical center
# positive above origin, negative below origin
slot_vertical_center = {value = [<num>,<denom>], original_unit = <str>}

# SVG files for start, end and middle of mounting rail
# minimum rail length should be set to the length of the
# start and end SVGs to not cause graphical issues
# if minimum rail length is not set, the middle SVG
# might get cut off unexpectedly.
#
# the start, middle and end images should not have lines where they join
# so when the images are placed together, there is no overlap.

start_image = <svg>

middle_image = <svg>

end_image = <svg>

# Catalog subtable for each mounting_rail_type. Groups common properties
# All fields here are optional, but highly encouraged.
[mounting_rail_types.<str>.catalog]

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

#### Equipment
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

# optional
# ID of enclosure instance
enclosure = <str>

# optional
# enclosure must also be defined
# ID of mount point (within an enclosure)
mount_point = <str>

# optional description
description = <str>

# Physical Location Information
[equipment.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>


[equipment.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[equipment.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

#### Wires
```toml
# dictionary of wires defined in project
# wires can only have two ends
# Wires within cables are assigned IDs automatically and are not listed here
[wires]

# table of attributes for wire instance
[wires.<str>]

# ID of wire type
wire_type = <str>

# structured name / wire number
identifier = <str>

# optional description
description = <str>

# ID of containing pathway instance
pathway = <str>

# wire length
length =  {value = [<num>,<denom>], original_unit = <str>}

# will be checked for 1 pin only
# intended for things like ferrules, ring terminals, etc.
end1_connector_type = <str>

end2_connector_type = <str>

# Physical Location Information
[wires.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

[wires.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[wires.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

#### Cables
```toml
# table of all cables within project
[cables]

# table of attributes on cable instance
[cables.<str>]
# ID of cable type
cable_type = <str>

# structured name / cable number
identifier = <str>

# optional description
description = <str>

# ID of pathway instance
pathway = <str>

length =  {value = [<num>,<denom>], original_unit = <str>}

# Physical Location Information
[cables.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

[cables.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[cables.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

#### Term Cables
```toml
# table of all term_cables in project
[term_cables]

# table of attributes on a pathway instance
[term_cables.<str>]

# ID of term_cable type
term_cable_type = <str>

# structured name / cable number
identifier = <str>

# optional description
description = <str>

# ID of pathway instance
pathway = <str>

# Physical Location Information
[term_cables.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

[term_cables.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[term_cables.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

#### Pathways
```toml
# Table of all pathways defined in project
[pathways]

# Table of attributes on a pathway instance
[pathways.<str>]

# ID of pathway type
pathway_type = <str>

# structured name / pathway identifier
identifier = <str>

# optional description
description = <str>

length =  {value = [<num>,<denom>], original_unit = <str>}

# Physical Location Information
[pathways.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

[pathways.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[pathways.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

#### Enclosures
```toml
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

# Physical Location Information
[enclosures.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

[enclosures.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[enclosures.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>

# dictionary of tables of sublocations/mounting locations within the enclosure
# used to represent DIN rail, or just specific coordinate locations in a specific location

# TODO: finish this
# Individual mount_points are defined as an enum with the following options:
# - CoordinatePair => Represents a coordinate pair on the backplane
# - MountingRail => Represents a mounting rail installed on a backplane
# individual DIN rails on a backplane, and then the distance along the DIN rail
# individual keystone slots on a panel
# rack units / sub rack units within a rack
[enclosures.<str>.mount_points.CoordinatePair.<str>]

# optional mounting rail id
# this ID must be defined in the project.
mounting_rail_id = <str>

# distance from left side of parent enclosure or location
x =  {value = [<num>,<denom>], original_unit = <str>}

# distance from bottom of parent enclosure
y =  {value = [<num>,<denom>], original_unit = <str>}

# distance along left side of location or rail
# allows you to not have to specify another sub-location for every single rail mounted component
distance = {value = [<num>,<denom>], original_unit = <str>}
```

#### Terminal Strips
```toml
# table of all terminal strips defined in the project
# all terminal blocks are part of a terminal strip
# a terminal strip is a collection of one or more terminal blocks
[terminal_strips]

# table of attributes for a specific terminal strip
[terminal_strips.<str>]

# structured name/tag strip ID / terminal strip name
identifier = <str>

# containing enclosure id
enclosure = <str>

# mounting rail id
mounting_rail = <str>

# Physical Location Information
[terminal_strips.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

[terminal_strips.<str>.iec_codes]
location = <str>
installation = <str>

# custom fields for user specified data. Not parsed
[terminal_strips.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>

# array of tables defining individual terminal blocks
# in terminal_strip.
# Definitions proceed left to right, horizontally.
[[terminal_strips.<str>.terminals]]

# number used for display order, defined left to right
terminal_number = <int>

# structured name / terminal number
identifier = <str>

# optional descriptive label
label = <str>

# terminal functional accessories
# These are things like fuses/fuse holders, component holders,
# lights, etc.
# <str> is accessory_type_id
accessories = [<str>]


# TODO: this should probably be an embedded table
# defining either terminal or terminal_strip_accessory type
# must be defined under the defintion of the terminal_block array it applies to
# second <str> can either be `Terminal` or `Accessory`
[terminal_strips.<str>.terminals.<str>]

# ID of terminal_block_type or terminal_strip_accessory_type
component_type = <str>

# array of jumpers defined in terminal strip
# these are only jumpers that exist within
# one terminal strip.
# wire jumpers that cross terminal strips
# should be defined as wires
[[terminal_strips.<str>.jumpers]]

# id of jumper type
jumper_type = <str>

# structured name / terminal number
identifier = <str>

# optional descriptive label
label = <str>

# array of `terminal_number`s as defined in the terminals array
# that indicate which terminals within a terminal strip
# this jumper connects
# can optionally have the terminal layer indicated with a dot and
# the terminal layer designation, allowing for multi-layer jumpers
jumper_connections = [<str>]
```

#### Mounting Rails
```toml
# list of mounting rails in project
[mounting_rails]

# table of attributes for a specific mounting rail
[mounting_rails.<str>]

mounting_rail_type = <str>

length = {value = [<num>,<denom>], original_unit = <str>}

[mounting_rails.<str>.iec_codes]
location = <str>
installation = <str>

# Physical Location Information
[mounting_rails.<str>.physical_location]

street_address = <str>
city = <str>
state = <str>
zip_code = <str>
latitude = [<num>, <denom>]
longitude = [<num>, <denom>]
structured_location_id = <str>
planet = <str>
building = <str>

# custom fields for user specified data. Not parsed
[mounting_rails.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

#### Connections
```toml
# Connections between two objects, commonly either wires/cables/term_cables and a terminal/connector on equipment
# This is the only root level item in the project definition that is an array rather than a table with sub-tables
# This is because there are no human generated identifiers. Individual connections are tracked internally.

# <str> for both end1 and end2 are dot joined ids of the specific objects
# for example to connect a wire within a cable to a connection on a terminal block
# TODO finish this example
[[connections]]

end1 = <str>

end1_type = <str>

end2 = <str>

end2_type = <str>


# This dictionary contains schematic symbols that represent equipment and components
# in the project
[schematic_symbols]

# Table of attributes for a specific instance of a symbol
[schematic_symbols.<str>]

symbol_type = <str>

symbol_color = <str>

# What this symbol represents.
# The type field must be filled in with Equipment, Terminal, or Connector
# and the value field must be filled in with an ID of a matching project component
represented_object = {type = <str>, value = <str>}

# custom fields for user specified data. Not parsed
[schematic_symbols.<str>.user_fields]
user0 = <str>
user1 = <str>
user2 = <str>
user3 = <str>
user4 = <str>
user5 = <str>
user6 = <str>
user7 = <str>
user8 = <str>
user9 = <str>
```

TODO: schematic symbols, drawings

