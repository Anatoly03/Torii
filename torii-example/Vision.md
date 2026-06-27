# Torii Vision

The long-term vision for the Torii project is to create a "world-building operating system" which will help story writers, book authors, hobbyist world builders with "documentating the encyclopedia of their own fantasy".

### Records

The idea is simple: Every element of a story is a "record", for example the character "Sarah Vermillion", the location "Adamant Mounts", the city of "Löwenherz", the in-world playing card deck "Trial Cards" and the constructed language "Kirlin".



Every thing that has its' own page in the encyclopedia of your fantasy is a "record" and every record is represented by a name and at least one file in the workspace directory: Usually it has the markdown files, but every image you add to the encyclopedia is also a record.

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

### Infoboxes &amp; Templates

There should be a way to "create" new components within a workspace, through templates or cloning existing components. For encyclopedia articles the idea is that we split "Article" into different sections (intuitively every time you add a new subtitle, make that section an entire component instead). "Fandom"-style infoboxes could be used as templates to create new components.

Imagine the component "Character Metadata" which is an infobox to the right having fields like "Titles", "Affiliations" or "Gender". By having these universal fields, every record which implements "Character Metadata" will have these fields ready to be defined.

### Queries &amp; Data Handling

The more technical people will have figured out that the record-component relation is inspired by ECS data design. This comes with the ability to query records.

Let's say you want to know the average age of all characters in your world. Since we have organized the infobox "Character Metadata" with an age field, we can query all characters programmatically and take the average of this fields' value. This is a rather specific information, but querying also includes the ability to:

- List all members of an organization (see if organization is in "Affiliated").
- List all currently alive characters in the story (see if not "deceased").
- Find all characters present in chapter 3.
- Find all records that have an image component and display them in the gallery.
- Generate family trees and dynasty relations.
- Generate the timeline of all events occuring in the story.

The idea of queries is to use structured data and get an understanding of their relations. You can also split the book into smaller "Chapters" and then simply collect it all into one file.

**With Torii we want to give world builders tools, not tell you how to use them!**

