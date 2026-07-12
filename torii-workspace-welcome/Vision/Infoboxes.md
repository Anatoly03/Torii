# Infoboxes &amp; Templates

A common component you can see on Wikipedia-type encyclopedias are organized infoboxes. The idea is to have an "infobox template" component which can be attached to a record, making it a reusable component in the workspace.

```
├── Character Metadata       |
│   ├── Title                | text
│   ├── Age                  | number
│   └── Species              | reference to "Species Metadata"
├── Species Metadata         |
│   ├── Allied               | reference
│   └── Enemies              | reference
└── Artefact Metadata        |
    ├── Latin Name           | text
    ├── Rarity Class         | select: "C", "B", "A", "S"
    └── ...
```

Imagine we have the record "Character Metadata" an we attach the component "Infobox" to it. Now, "Character Metadata" is not only a record in the workspace, but a component which can be reused in the workspace.

Now, when "Character Metadata" is attached to the record "Emma Rosenfeld", she will get an infobox aligned to the right, with the attributes "Title" or "Age" ready to be defined.

