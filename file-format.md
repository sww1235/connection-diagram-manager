# File Formats

All file formats are currently based on YAML, (and inspired by
[WireViz](https://github.com/formatc1702/WireViz)).

Connection Diagram Manager uses a directory structure to represent an
individual project. All YAML files in the `src` subdirectory will be parsed for
any of the following information. It can be all in one file, or split up into
multiple files. Each file must not contain duplicate top level dictonaries due
to parsing limitations, howeverthey can be duplicated across multiple files. (I
didn't want to write a custom deserializer). If there are unique values that
conflict, the user will be notified with the file names and line numbers that
caused the conflict.

In addition, additional files can be specified in the project config
file that can contain a limited subset of top level dictionaries that contain
library type information such as `connector_type`, `equipment_type`, etc.

Due to the support for multiple files, (and also the possibility of
supporting alternate file syntax formats), YAML references are
not used, unlike in WireViz.

Some libraries will be autopopulated with default values when the program
starts. An optional flag, provided either in the project config file, or on the
command line will prevent this and only use the values loaded from files.

On import, all project data will be validated against the library definitions.
If values are found in the project that are not present in the library, the
user will be prompted on load if they want to save the value to the global
library or a local project library, replace the value with one already in the
libraries, ignore the value, or cancel the load. All errors will indicate file
name and line number so the error is easy to fix. If the user elects to save to
either the global or local library, the load process will be paused and the
user prompted for any additional information needed.


## Project Configuration File

This file must be named `cdm_config.yaml` and be in the project `src` or root
directory. If multiple `cdm_config.yaml` files are found, the one in `src` will
be used to avoid conflict with any other config files placed in the root
directory such as for linters or automated processing of some kind.
```yaml
library_files: <list>
no_default_libraries: <bool>
```


## Library Definitions
```yaml
connector_type: # dictionary of all available connector types

	<str>: # connectorType designator (must be unique)

		# most tags are optional and will be filled with
		# blank values if unpopulated

		manufacturer: <str>			# manufacturer name
		model: <str>				# connector model description
		description: <str>			# free text field for larger descriptions of connectors
		mount_type: <str>			# cable, pcb through hole, pcb surface mount, panel
		panel_cutout: <str>			# D, A, etc. optional
		pn: <str>					# [internal] part number
		mpn: <str>					# manufacturer part number
		supplier: <str>				# supplier
		spn: <str>					# supplier part number
		gender: <str>				# (male, female, rpmale, rpfemale, hermaphroditic, unknown)
		height: <float>				# height of connector in mm
		width: <float>				# width of connector in mm
		depth: <float>				# depth of connector in mm
		diameter: <float>			# diameter of circular connector in mm

		# pinout information
		pincount: <int>				# if omitted, is set to length of specified list(s)
		# at least one of the following must be specified
		pins: <list>				# if omitted, is autofilled with [1, 2, ..., pincount]
		pinlabels: <list>			# if omitted, is autofilled with blanks
		# pin color marks (optional)
		pincolors: <list>			# list of colors to be assigned
									# goes in order of pin count/pin list
									# if fewer colors are specified than pins, end of list will have no colors specified
		pin_signal_type: <list>		# same specs as pincolors

		# all images are specified as SVG images so they scale.
		visrep: <svg>				# svg text (ideally minimized), can be multiline
		pin_visrep: <svg>			# svg text, shows pin layout of connector


connector_type_mate:	# dictionary of which connectors mate with which other connectors
						# https://dba.stackexchange.com/a/48663/185504
						# TODO:

equipment_type: # dictionary of all available equipment types
	<str>: # equipmentType designator (must be unique)

		# most tags are optional and will be filled with
		# blank values if unpopulated

		manufacturer: <str>
		model: <str>
		pn: <str>					# [internal] part number
		mpn: <str>					# manufacturer part number
		supplier: <str>				# supplier
		spn: <str>					# supplier part number
		mounting_type: <list>		# (19" rack, 23" rack, 1/2 19" rack, DIN rail,
									# surface wall mount, inset wall mount, panel, custom)
		type: <str>					# (audio, video, mix, lighting, networking, patch panel, power)
		faces:	# dictionary of faces that can have connectors associated with them,
				# and an associated visual representation. should not include connectors themselves.
			<str>: <svg>
			<str>: <svg>
		visrep: <svg>				# overall visual representation of equipment

		connectors: # dictionary of connectors on equipment. Accessed via dot notation

			<str>: # connectorType identifier

				id: <str>			# identifier of connector on equipment. must be unique per equipment type
				direction: <str>	# (input, output, power input, power output, bidirectional, passive)
				face: <str>			# face connector is located on
				x: <integer>		# location of connector from bottom left of visrep of face to right
				y: <integer>		# location of connector from bottom left of visrep of face up

# will be populated with a default set of values unless otherwise specified.
pathway_type:	# dictonary of all available cable pathway types.
				# This is used for things like conduit, and cable tray,
				# but also includes things like J-hooks, or free-air cables.
				# Some of these definitions may be fluid and can be configured as the user desires.
	<str>:		# pathway type designator (must be unique)
		type: <str>					# type of cable pathway (conduit, cable tray, etc)
		pn: <str>					# [internal] part number
		mpn: <str>					# manufacturer part number
		supplier: <str>				# supplier
		spn: <str>					# supplier part number
		size: <str>					# specified and parsed differently depending on type
									# TODO: need to define specific size defintions
									# files use metric units
		trade_size: <str>			# include both metric and standard trade sizes in string
		cross_sect_area: <float>	# specified in mm^2. Can be displayed in circular mills in application
		material: <str>				# primary material of pathway


wire_type:	# dictonary of all available wire types.
			# A wire is defined as a material (not necessarily conductive) with optional insulation.
			# if a product has a shield or additional layers, it must be defined as a cable
			# insulation color is defined on individual wire instance
			# wire, cable and term_cable designators must all be unique
			#
	<str>:	# wire type designator (must be unique)
		material: <str>				# copper, alumninum, ACSR, steel, glass, plastic
		manufacturer: <str>
		pn: <str>					# [internal] part number
		mpn: <str>					# manufacturer part number
		supplier: <str>				# supplier
		spn: <str>					# supplier part number
		insulated: <bool>
		insulation_material: <str>	# PVC, Nylon, thermoplastic, etc
		wire_type_code: <str>		# THWN, XHHN, etc
		overall_cross_sect_area: <float> # including insulation, specified in mm^2
		conductor_ cross_sect_area: <float>	# the cross sectional area of the conductor, specified in mm^2.
		stranded: <bool>
		num_strands: <int>			# number of strands if cable is stranded. defaults to 1 if cable is solid
		insul_volt_rating: <float>	# voltage rating of insulation.
		insul_temp_rating: <float>	# temperature rating of insulation. Specified in degrees centigrade.

cable_type: # dictonary of all available raw cable types.
			# A cable is defined as one or more wires mechanically attached together,
			# with optional insulation and semiconducting layers, and optional shields
			# if a product has a shield or additional layers, it must be defined as a cable
			# wire insulation color is defined on individual wire instance
			# individual wire instances within cable are accessed with dot notation
			# wire, cable and term_cable designators must all be unique
			#
	<str>:	# cable type designator (must be unique)
		core:	# dictionary of wire or cable cores inside cable.
				# strength members are treated as a wire
			<str>: # identifier of individual core. Must be unique per cable_type.
				type: <str>			# identifier of wire/cable type that core is
				is_wire: <bool>		# is this core a wire?
		manufacturer: <str>
		pn: <str>					# [internal] part number
		mpn: <str>					# manufacturer part number
		supplier: <str>				# supplier
		spn: <str>					# supplier part number
		cable_type_code: <str>		# SOOW, FC, FCC, TC, MC, AC, MC, UF, PLTC, MV, etc
		cross_sect_area: <float>	# specified in mm^2. Outer area of cable
		cross_section: <str>		# oval, circular, siamese
		height: <float>				# height of cable if oval or siamese, specified in mm
		width: <float>				# width of cable if oval or siamese, specified in mm
		diameter: <float>			# diameter of cable if circular, specified in mm

		layer: # list of shields and insulation layers on outside of cable
			layer_nbr: <int>			# counted from inside to outside of cable
			layer_type: <str>				# insulation, semiconductor, shield, screen, concentric neutral
			material: <str>
			volt_rating: <float>	# voltage rating for insulation layer
			temp_rating: <float>	# temp rating for insulation layer. Specified in degrees centigrade
			color: <str>			# color of insulation or semiconductor

term_cable_type:	# dictionary of available manufactuered cables,
					# consisting of a raw cable or wire type and connector specifications.
					# term cables can only have two ends, but each end can have
					# a fan out or split with multiple connectors
					# connectors defined on a term_cable are accessed based on dot notation
					# wire, cable and term_cable designators must all be unique
	- manufacturer: <str>		# Manufacturer of term_cable
	  pn: <str>					# [internal] part number
	  mpn: <str>				# manufacturer part number
	  supplier: <str>			# supplier
	  spn: <str>				# supplier part number
	  cable: <str>				# ID of cable. Only one of wire or cable can be specified
	  wire: <str>				# ID of wire
	  nom_length: <float>		# nominal length in meters
	  length: <float>			# actual length in meters
	  end1:						# dictionary of connectors attached to term cable
		- connector_type: <str>			# ID of connector type
		  termination:			# dictionary of core to connector pin mappings for each connector
								# manual termination between pin and core must be specified
			- core: <str>
			  pin: <str>

	  end2:						# dictionary of connectors attached to term cable
		- connector_type: <str>			# ID of connector type
		  termination:			# dictionary of core to connector pin mappings for each connector
								# manual termination between pin and core must be specified
			- core: <str>
			  pin: <str>


location_type:	# dictionary of available location types
	<str>:		# unique ID of location type
		manufacturer: <str>			# Manufacturer of term_cable
		pn: <str>					# [internal] part number
		mpn: <str>					# manufacturer part number
		supplier: <str>				# supplier
		spn: <str>					# supplier part number
		width: <float>				# overall width of location, specified in mm
		height: <float>				# overall height of location, specified in mm
		depth: <float>				# overall depth of location, specified in mm
		usableWidth: <float>		# usable internal width of location, specified in mm
		usableDepth: <float>		# usable internal depth of location, specified in mm
		usableHeight: <float>		# usable internal height of location, specified in mm.



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


colors: # dictionary of colors. The color name (key) must be unique.
	<str>: <str>					# name: abbreviation


```



the difference between `cables` and `term_cables` is that `term_cables` are manufactured to a specific nominal length and `cables` are custom terminated.

## Project Definitions

```yaml
equipment:		# dictionary of equipment defined in project
	<str>:		# unique ID of equipment instance
		type: <str>					# ID of equipment type
		identifier: <str>			# structured name
		mounting_type: <str>		# must be in list of mounting types defined on equipment type
		location: <str>				# ID of location instance
		description: <str>			# optional description


wires:			# dictonary of all wire instances defined in project
				# wires can only have two ends
				#
	<str>:		# unique ID of wire instance.
		type: <str>					# ID of wire type
		identifier: <str>			# structured name
		description: <str>			# optional description
		pathway: <str>				# ID of pathway instance
		length: <float>				# length in meters
		end1:						# connector attached to wire
			type: <str>				# ID of connector type
		end2:						# connectors attached to wire
			type: <str>				# ID of connector type


cables:			# dictonary of all cable instances defined in project
				# cables can only have two ends, but each end can have
				# a fan out or split with multiple connectors
				#
	<str>:		# unique ID of cable instance.
				# Wires within cables are assigned IDs automatically and are not listed here
		type: <str>					# ID of cable type
		identifier: <str>			# structured name
		description: <str>			# optional description
		pathway: <str>				# ID of pathway instance
		length: <float>				# length in meters
		end1:						# dictionary of connectors attached to cable or wire
									# technically optional but being excluded will cause
									# connections specified to be flagged as errors
			type: <str>				# ID of connector type
			autoTerm: <str>			# auto termination method, current available values are:
									# `pin_core` which matches numbered or unique named pins and cores with each other
									# others to be thought of at a later date.
			termination:			# dictionary of core to connector pin mappings for each connector
									# either auto termination method or manual termination method
									# must be specified
		end2:						# dictionary of connectors attached to cable or wire
									# technically optional but being excluded will cause
									# connections specified to be flagged as errors
			type: <str>				# ID of connector type
			autoTerm: <str>			# auto termination method, current available values are:
									# `pin_core` which matches numbered or unique named pins and cores with each other
									# others to be thought of at a later date.
			termination:			# dictionary of core to connector pin mappings for each connector
									# either auto termination method or manual termination method
									# must be specified

term_cables:	# dictonary of all term_cable instances defined in project
				# term_cables can only have two ends, but each end can have
				# a fan out or split with multiple connectors
				#
	<str>:		# unique ID of term_cable instance.
				# Wires within cables are assigned IDs automatically and are not listed here
		type: <str>					# ID of term_cable type
		identifier: <str>			# structured name
		description: <str>			# optional description
		pathway: <str>				# ID of pathway instance


pathways:		# dictonary of pathways defined in project
	<str>:		# unique ID of pathway
		type: <str>					# ID of pathway type
		identifier: <str>			# structured name
		description: <str>			# optional description
		length: <float>				# length in meters

locations:		# dictionary of locations defined in project
				# locations may have sublocations defined in them.
				# examples of sublocations would be coordinate pairs on a backplane,
				# individual DIN rails on a backplane, and then the distance along the DIN rail
				# individual keystone slots on a panel
				# rack units / sub rack units within a rack
				#
				# Need to check sublocations for recursion
				#
	<str>:		# unique ID of location instance
		type: <str>					# ID of location type
		identifier: <str>			# structured name
		description: <str>			# optional description
		phyiscalLocation: <str>		# street address, coordinates, description
		sublocations:				# dictionary  of sublocations
			id:	<str>				# unique id of location, no recursion
			x: <float>				# distance from left side of parent location, specified in mm
			y: <float>				# distance from bottom of parent location, specified in mm
			z: <float>				# distance from back of parent location, specified in mm


connection:		# list of all connections defined in project, with submappings to identify the objects that are connected
				# connections are uniquely identified by concatenating the two ids of the connected objects together
				#
	- end1: <str>					# unique identifier of connected object.
									# If connected object contains subobjects, and they are not specifically
									# connected together, but their parents are, application logic will assume
									# connection patterns for the subobjects.
	- end2: <str>					# unique identifier of connected object. Cannot be the same as end1.

```


