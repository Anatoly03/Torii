# Components

The following components are currently implemented.

### "Article" Component

- Create, View, Edit

Every markdown note is an "Article". It is a what-you-see-is-what-you-get type editor optimal for lore and handcrafted records.

- The component can link other "Articles": When writing the name of an existing record in-article, you can tab or enter it to create a hyperlink.
- The component can display other "Images": **TBA**

For example I have a unique card deck specifically for my world which I call the "Crown Cards", they consist of 80 cards, and while in theory I could make 80 records, it's most simple to simply put an article and define everything there. 

### "Image" Component

- View, Delete

Every "image media" in the workspace is a record with an image component. Similar to "Wikimedia", you can write metadata about the image. An app user can make the decision between "coupling" an image with a record or splitting it.

- The image component can not link other components and has no special interactions with them.

For example I have an organization called "Teulu Guild", of which "Sarah Vermillion" is a high-ranking member. I have an "Article" and an "Image" for Sarah, with the image being a character icon. But the guild has been through many logo changes over the centuries in my world, so I only implement "Article" for "Teulu Guild" and the images are linked in the article (but they are different records).

