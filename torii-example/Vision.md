# Torii Vision

The long-term vision for the Torii project is to create a "world-building operating system" which will help story writers, book authors, hobbyist world builders with "documentating the encyclopedia of their own fantasy".

### Records

The idea is simple: Every element of a story is a "record", for example the character "Sarah Vermillion", the location "Adamant Mounts", the city of "Löwenherz", the in-world playing card deck "Trial Cards" and the constructed language "Kirlin".



Every “thing" that has its' own page in the encyclopedia of your fantasy is a "record" and every “record" is represented by at least a name and one file in the workspace directory: Usually it has the markdown note where you can write prose and lore, but every image, every map and every object you add to the encyclopedia is also a record.

### Component

Let's say you have the map layout of the "Pirate Bay" island in mind. You can write about it in the markdown file, but it is limited at text processing and cannot draw you the island. A "map" component would be attached to the Pirate Bay and you would be able to both "draw" and "document" the "record" "Pirate Bay".

In other words, "Pirate Bay" is a "record". We do not know wether it is an image, or an article, or a language, or a location, or anything else. We add "components" which defined the nature and behaviour of this record. The "components" would be: "Map" and "Article".



For more complex records "Article" is also often a collection of multiple chapters, if we want to track history of many records, we could split "Article" into "History", "Biome" and "Trivia", their own components!



"Record" is an atomic element of the world. "Component" is an atomic reusable pattern on elements.

### "Article" Component

An Article is the most intuitive component: It's a wall of text you can edit, format and display.

The first sentence of the article is used to generate the special "Brief" component. Every record which defines "Brief" can be queried for and used for display in the main view gallery.

The file pattern is `<entity>.md`.

### "Image" Component

The Image component treats graphical media attached to the workspace. For example when you add the image "Screenshot", the record "Screenshot" comes with the component "Image".

Images can be inserted into the article body too, but then it is always a reference to a different record.

The file pattern is `<entity>.png` for PNG files, but this component also manages other image-related file extensions.

At first images are static, but in the long run we should add powerful in-built editors for rastor and vector images. Manga/ comics authors would benefit from this.

### "Language" Component

The language component is used to document languages used in the world. These can either be constructed languages, or language dialects.

A language in theory defines an alphabet (= script), vocabulary, grammar. To create new symbols, you would create new records (e.g. "Letter A") and link them in the language. Grammar could be documented in the article.

An implementation question is how the language could be reused in the article component, but that's a technical question for later. We should create an LLM-powered translator on the API server: Users (who volunteer) will be able to translate English to their conlang and reverse.

The file pattern is `<entity>.language.yaml`.

### "Map" Component

The map component is a geopolitical slice of the map.

The file pattern is `<entity>.map.yaml`.

The following layers should be considered, discussion is needed on which of these can be abstracted to be their own components/ records:

- Surface: Heightmap (land vs water)
- Surface: Biomes (forest, desert, etc.)
- Meta: Scale. (Since a map can be reused into another map, make sure the scale is consistent.)
- Marker "Location": Reference to another record as a "position".
- Marker "Area": Reference to another record as an "areal" zone, such as political borders and cultural landscapes.
- Marker "Path": Reference to another record, such as rivers or highways.

### "Time System" Component

Some worlds lead their own calendars. For authors who deeply care about proper time management, it is important to allow defining their own time system formats, following simple rules:

- Repeat "Day"
- Repeat "Week" Every 7 Days with Day 1 = Monday, etc.
- To-Do: Think about months, years

### "Scene" Component

Fragment of a chapter, a scene is an article describing events occuring in a certain place (record), at a certain time in a time system (record) with certain characters present (records).

### Minor Component Ideas

Below are minor component ideas which are too small for their own subtitle.

- Manga Page Layout: Arrange images and speech bubbles onto a page.
- Quote Component: Could be used in the gallery to scroll beautiful quotes from the world.
- Split "Identity" and "Character": Some villains live two-lifes.
- Chapter: Collection of Scenes.
- Port “Azgaars’ Fantasy Map” if possible to a map component.

### Infoboxes &amp; Templates

There should be a way to "create" new components within a workspace, through templates or cloning existing components. For encyclopedia articles the idea is that we split "Article" into different sections (intuitively every time you add a new subtitle, make that section an entire component instead). "Fandom"-style infoboxes could be used as templates to create new components.

Imagine the component "Character Metadata" which is an infobox to the right having fields like "Titles", "Affiliations" or "Gender". By having these universal fields, every record which implements "Character Metadata" will have these fields ready to be defined.



When a plugin called `<plugin>` defines a component, we should consider the component file extension to be `<entity>.<plugin>.<component>.<extension>` to avoid ambiguity between components (e.g. “map” and “azgaar.map” and other map-like implementations)

### Queries &amp; Data Handling

The more technical people will have figured out that the record-component relation is inspired by ECS data design. This comes with the ability to query records.

Let's say you want to know the average age of all characters in your world. Since we have organized the infobox "Character Metadata" with an age field, we can query all characters programmatically and take the average of this fields' value. This is a rather specific information, but querying also includes the ability to:

- List all members of an organization (see if organization is in "Affiliated").
- List all currently alive characters in the story (see if not "deceased").
- Find all characters present in chapter 3.
- Find all records that have an image component and display them in the gallery.
- Generate family trees and dynasty relations.
- Generate the timeline of all events occuring in the story.

The idea of queries is to use structured data and get an understanding of their relations. You can also split the book into smaller "Chapters" and then simply collect it all into one file. **With Torii we want to provide world builders with powerful tools, not tell you how to use them!**

### Version Controlling &amp; File System

The Torii ecosystem is specifically designed to be compatible with GIT and the local filesystem.. All components are individual files that can be committed and tracked, but also individually removed and managed. 

When possible, components should not be custom binaries but represented as a human-readable git-frienly data format (such as JSON and YAML). When porting data formats to Torii components this is not always possible (Images or if someone ports Azgaars’ Map engine to Torii).

### "Fun" Vision: Godot Games in Torii

Since the entire concept of Torii is compatible with Godot we could make a fun "demo" of a simple game generator in Torii, in the long-term. The idea is to map Torii record-components to Godot entity-components and compile a prototype.

- The "Article" component will map to a very detailed documentation comment for an entity, some special components to map to Godot components will be needed.
- "Image" will map to a sprite.

These special components would be useful:

- "Character" which consists of a character image and replaces the image at the top, it's a simple editor with a hitbox radius, and animations. Below the character preview will be some buttons (I'm thinking of assets which look like "stone" buttons) to trigger a certain animation.
- "Tiles" as individual records containing one or multiple tiles, with or without hitbox.
- "Level" using tile records and being able to place characters.

And no more, this should be a simple demo. Just a player in a simple level colliding with the world.



&nbsp;

&nbsp;

&nbsp;