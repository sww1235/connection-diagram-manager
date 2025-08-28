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
default_library_files = [<path string>]

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

TODO: add component tag format and options, wire number format and options, cross-reference options, styles (arrow, plc, fan-in/fan-out, wire cross, wire tee, wire connection,

```toml
# Required
project_name = <str>

# optional
load_default_libraries = <bool>

# optional
# Paths can be either relative or absolute
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

**NOTE:** Any number that is not specifically an integer, is implemented internally as a `[Rational64]` to work around precision issues with floats. You must specify both a numerator and denominator in the array or you will get an error. As an example, if you wanted to represent the number 1/3, a float of 0.333333... isn't exact. With Rational types, you can specify it as exactly 1/3 and be satisfied. Floating point numbers are still used, especially to produce decimal output from Rational types but all the math internally is done with Rational types and just the output step is converted.

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

# free text field for larger descriptions of connectors
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
mounting_type = <list>

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

# free text field for larger descriptions of equipment type
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

# dictonary of all available wire or cable pathway types.
# This is used for things like conduit, and cable tray,
# but also includes things like J-hooks, or free-air cables.
[pathway_type]

# table (dictionary) representing one pathway type
# The `<str>` is the pathway type identifier. This is a `key` in TOML and
# must comply with the TOML spec.

# Most keys in a pathway_type sub-table are optional
[pathway_type.<str>]

# type of cable pathway (conduit, cable tray, etc)
type = <str>

# specified and parsed differently depending on type
size = <str>

# optional
trade_size = <str>

# optional
# Interior cross sectional area - used for conduit fill calculations
cross_sect_area =  [<num>,<denom>]

# primary material of pathway
material = <str>


# Catalog subtable for each pathway_type. Groups common properties
# All fields here are optional, but highly encouraged.
[pathway_type.<str>.catalog]

# manufacturer name
manufacturer = <str>

# equipment type model description
model = <str>

# free text field for larger descriptions of equipment type
description = <str>

# [internal] part number
part_number = <str>

# manufacturer part number
manufactuer_part_number = <str>

# supplier
supplier = <str>

# supplier part number
supplier_part_number = <str>


wire_type =	# dictonary of all available wire types.
			# A wire is defined as a material (not necessarily conductive) with optional insulation.
			# if a product has a shield or additional layers, it must be defined as a cable
			# insulation color is defined on individual wire instance
			# wire, cable and term_cable designators must all be unique
			# //TODO: move color to wire_type definition as it affects the mpn/spn/etc
			#
	<str> =	# wire type designator (must be unique)
		material = <str>				# copper, alumninum, ACSR, steel, glass, plastic
		manufacturer = <str>
		part_number = <str>					# [internal] part number
		manufacturer_part_number = <str>					# manufacturer part number
		supplier = <str>				# supplier
		supplier_part_number = <str>					# supplier part number
		insulated = <bool>
		insulation_material = <str>	# PVC, Nylon, thermoplastic, etc
		wire_type_code = <str>		# THWN, XHHN, etc
		insulation_thickness =  [<num>,<denom>] # specified in mm
		overall_cross_sect_area =  [<num>,<denom>] # including insulation, specified in mm^2
		conductor_ cross_sect_area =  [<num>,<denom>]	# the cross sectional area of the conductor, specified in mm^2.
		stranded = <bool>
		num_strands = <int>			# number of strands if cable is stranded. defaults to 1 if cable is solid
		insulation_volt_rating =  [<num>,<denom>]	# voltage rating of insulation.
		insulation_temp_rating =  [<num>,<denom>]	# temperature rating of insulation. Specified in degrees centigrade.
		insulation_color = <str> 			# insulation color. Pick from following options:
											# GREEN, BLACK, WHITE, RED, BLUE, YELLOW, ORANGE, BROWN,
											# PINK, PURPLE, SLATE, ROSE, VIOLET, AQUA, #RRGGBB (hex color code)
		secondary_insulation_color = <str> 	# same as insulation color

cable_type = # dictonary of all available raw cable types.
			# A cable is defined as one or more wires mechanically attached together,
			# with optional insulation and semiconducting layers, and optional shields
			# if a product has a shield or additional layers, it must be defined as a cable
			# wire insulation color is defined on individual wire instance
			# individual wire instances within cable are accessed with dot notation
			# wire, cable and term_cable designators must all be unique
			#
	<str> =	# cable type designator (must be unique)
		core =	# dictionary of wire or cable cores inside cable.
				# strength members are treated as a wire
			<str> = # identifier of individual core. Must be unique per cable_type.
				type = <str>			# identifier of wire/cable type that core is
				is_wire = <bool>		# is this core a wire?
		manufacturer = <str>
		part_number = <str>					# [internal] part number
		manufacturer_part_number = <str>					# manufacturer part number
		supplier = <str>				# supplier
		supplier_part_number = <str>					# supplier part number
		cable_type_code = <str>		# SOOW, FC, FCC, TC, MC, AC, MC, UF, PLTC, MV, etc
		cross_sect_area =  [<num>,<denom>]	# specified in mm^2. Outer area of cable
		cross_section = <str>		# oval, circular, siamese
		height =  [<num>,<denom>]				# height of cable if oval or siamese, specified in mm
		width =  [<num>,<denom>]				# width of cable if oval or siamese, specified in mm
		diameter =  [<num>,<denom>]			# diameter of cable if circular, specified in mm

		layer = # list of shields and insulation layers on outside of cable
			layer_number = <int>			# counted from inside to outside of cable
			layer_type = <str>				# insulation, semiconductor, shield, screen, concentric neutral, jacket
			material = <str>
			volt_rating =  [<num>,<denom>]	# voltage rating for insulation layer
			temp_rating =  [<num>,<denom>]	# temp rating for insulation layer. Specified in degrees centigrade
			color = <str>			# color of insulation or semiconductor

term_cable_type =	# dictionary of available manufactuered cables,
					# consisting of a raw cable or wire type and connector specifications.
					# term cables can only have two ends, but each end can have
					# a fan out or split with multiple connectors
					# connectors defined on a term_cable are accessed based on dot notation
					# wire, cable and term_cable designators must all be unique
	- manufacturer = <str>		# Manufacturer of term_cable
	  part_number = <str>					# [internal] part number
	  manufacturer_part_number = <str>				# manufacturer part number
	  supplier = <str>			# supplier
	  supplier_part_number = <str>				# supplier part number
	  cable = <str>				# ID of cable. Only one of wire or cable can be specified
	  wire = <str>				# ID of wire
	  nom_length =  [<num>,<denom>]		# nominal length in meters
	  length =  [<num>,<denom>]			# actual length in meters
	  end1 =						# dictionary of connectors attached to term cable
		- connector_type = <str>			# ID of connector type
		  terminations =			# dictionary of core to connector pin mappings for each connector
								# manual termination between pin and core must be specified
			- core = <str>
			  pin = <str>

	  end2 =						# dictionary of connectors attached to term cable
		- connector_type = <str>			# ID of connector type
		  terminations =			# dictionary of core to connector pin mappings for each connector
								# manual termination between pin and core must be specified
			- core = <str>
			  pin = <str>


location_type =	# dictionary of available location types
	<str> =		# unique ID of location type
		manufacturer = <str>			# Manufacturer of term_cable
		part_number = <str>					# [internal] part number
		manufacturer_part_number = <str>					# manufacturer part number
		supplier = <str>				# supplier
		supplier_part_number = <str>					# supplier part number
		width =  [<num>,<denom>]				# overall width of location, specified in mm
		height =  [<num>,<denom>]				# overall height of location, specified in mm
		depth =  [<num>,<denom>]				# overall depth of location, specified in mm
		usableWidth =  [<num>,<denom>]		# usable internal width of location, specified in mm
		usableDepth =  [<num>,<denom>]		# usable internal depth of location, specified in mm
		usableHeight =  [<num>,<denom>]		# usable internal height of location, specified in mm.



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
```



the difference between `cables` and `term_cables` is that `term_cables` are manufactured to a specific nominal length and `cables` are custom terminated.

### Project Definitions

```yaml
equipment =		# dictionary of equipment defined in project
	<str> =		# unique ID of equipment instance
		type = <str>					# ID of equipment type
		identifier = <str>			# structured name
		mounting_type = <str>		# must be in list of mounting types defined on equipment type
		location = <str>				# ID of location instance
		description = <str>			# optional description


wires =			# dictonary of all wire instances defined in project
				# wires can only have two ends
				#
	<str> =		# unique ID of wire instance.
		type = <str>					# ID of wire type
		identifier = <str>			# structured name
		description = <str>			# optional description
		pathway = <str>				# ID of pathway instance
		length =  [<num>,<denom>]				# length in meters
		end1 =						# connector attached to wire
			type = <str>				# ID of connector type
		end2 =						# connectors attached to wire
			type = <str>				# ID of connector type


cables =			# dictonary of all cable instances defined in project
				# cables can only have two ends, but each end can have
				# a fan out or split with multiple connectors
				#
	<str> =		# unique ID of cable instance.
				# Wires within cables are assigned IDs automatically and are not listed here
		type = <str>					# ID of cable type
		identifier = <str>			# structured name
		description = <str>			# optional description
		pathway = <str>				# ID of pathway instance
		length =  [<num>,<denom>]				# length in meters
		end1 =						# dictionary of connectors attached to cable or wire
									# technically optional but being excluded will cause
									# connections specified to be flagged as errors
			type = <str>				# ID of connector type
			autoTerm = <str>			# auto termination method, current available values are:
									# `pin_core` which matches numbered or unique named pins and cores with each other
									# others to be thought of at a later date.
			terminations =			# dictionary of core to connector pin mappings for each connector
									# either auto termination method or manual termination method
									# must be specified
		end2 =						# dictionary of connectors attached to cable or wire
									# technically optional but being excluded will cause
									# connections specified to be flagged as errors
			type = <str>				# ID of connector type
			autoTerm = <str>			# auto termination method, current available values are:
									# `pin_core` which matches numbered or unique named pins and cores with each other
									# others to be thought of at a later date.
			terminations =			# dictionary of core to connector pin mappings for each connector
									# either auto termination method or manual termination method
									# must be specified

term_cables =	# dictonary of all term_cable instances defined in project
				# term_cables can only have two ends, but each end can have
				# a fan out or split with multiple connectors
				#
	<str> =		# unique ID of term_cable instance.
				# Wires within cables are assigned IDs automatically and are not listed here
		type = <str>					# ID of term_cable type
		identifier = <str>			# structured name
		description = <str>			# optional description
		pathway = <str>				# ID of pathway instance


pathways =		# dictonary of pathways defined in project
	<str> =		# unique ID of pathway
		type = <str>					# ID of pathway type
		identifier = <str>			# structured name
		description = <str>			# optional description
		length =  [<num>,<denom>]				# length in meters

locations =		# dictionary of locations defined in project
				# locations may have sublocations defined in them.
				# examples of sublocations would be coordinate pairs on a backplane,
				# individual DIN rails on a backplane, and then the distance along the DIN rail
				# individual keystone slots on a panel
				# rack units / sub rack units within a rack
				#
				# Need to check sublocations for recursion
				#
	<str> =		# unique ID of location instance
		type = <str>					# ID of location type
		identifier = <str>			# structured name
		description = <str>			# optional description
		phyiscalLocation = <str>		# street address, coordinates, description
		sublocations =				# dictionary  of sublocations
			id =	<str>				# unique id of location, no recursion
			x =  [<num>,<denom>]				# distance from left side of parent location, specified in mm
			y =  [<num>,<denom>]				# distance from bottom of parent location, specified in mm
			z =  [<num>,<denom>]				# distance from back of parent location, specified in mm


connection =		# list of all connections defined in project, with submappings to identify the objects that are connected
				# connections are uniquely identified by concatenating the two ids of the connected objects together
				#
	- end1 = <str>					# unique identifier of connected object.
									# If connected object contains subobjects, and they are not specifically
									# connected together, but their parents are, application logic will assume
									# connection patterns for the subobjects.
	- end2 = <str>					# unique identifier of connected object. Cannot be the same as end1.

```


