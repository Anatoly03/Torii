# Cartograpy

A common practice in modern literature is to invent new maps which help when following the narrative of long-distance journeys. A map is, abstractly speaking, a canvas which portrays locations, paths, borders and zones. We do not know if seas exist in the users' imaginary world, perhaps the user has "oceans" under "other oceans", and above the "land" there could be "above-land".



There are three main "objects" a map can consist of.

- A **region**, a two-dimensional area portraying the difference between "part of" and "not part of", for example landmasses, biomes, geopolitics, cultural zones, etc.
- A **route**, a one-dimensional path on the canvas connecting two dots, for example a river, a street or a demilitarized border.
- A **location**, a non-dimensional singular point on the canvas, for example a city, a chest or a simple marker.

The map component would provide the tools to edit each of the three "objects" and assign them to an external records.



To enable the record hierarchy that Torii also focuses on, a map can be reinserted into another map. For example if I have a "Pirate Island", which is on the continent of "Kitania", which on the other hand is part of the global "World Map", then I have listed three different records: The child-record has no information about the parent.



We need to discuss "special objects", like heightmap or depth maps, because they don't fit into the "three objects". I'm also not sure about biomes and how to integrate map drawing with assets (e.g. mountains)



Additionally to the map objects, a map has some metadata:

- A **scale**, so the map inheritance preserves the appropriate size of records in their parents component.

The components' file pattern should be `<entity>.map.yaml`.

