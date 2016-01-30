Specification
=============
All resources are defined using the `TOML` [file format](https://github.com/toml-lang/toml) for more details. See `README.md` for more information on using resources.

Common Fields
-------------
Below are fields that are common between most resource type:

| Field         | Required | Type         | Description
|---------------|:--------:|--------------|-------------
| `name`        | ✓        | String       | The name of the element. This field is used by other fields to reference specific elements. The name is case-insensitive (e.g. 'A' is the same as 'a') and must be unique amongst all other elements. The name is used as an identifier, if and only if it is a valid identifier (it must only contains alphanumerical characters or '_' and has to start with a character that is not a digit). If it is an invalid identifier, you must include an `internal` field.
| `id`          | ✓        | Integer      | The identifying number for the resource. This is used for enumerations. It must be a number greater than or equal to 0 and less than the total number of resources for the resource type.
| `internal`    |          | String       | An identifier value for the element. It is only required if `name` is an invalid identifier. The value stored here must be a valid identifier (see `name`).

Elements
--------
Elements are elemental types that classify moves and species. All elements must have an `[element]` root. Uses the common fields.

| Field         | Required | Type         | Description
|---------------|:--------:|--------------|-------------
| `weaknesses`  |          | String Array | A list of other elements that this element is weak to. This represents a double effectiveness from the other elements when battling this element.
| `resistances` |          | String Array | A list of other elements that are resistant to this element to. This represents a half effectiveness from this element when battling the other elements.
| `immunities`  |          | String Array | A list of other elements that are immune against this element. This represents a non-effectiveness from this element when battling the other elements.

Species
-------
Species provide base data and meta-data for monsters.

### Root Fields
All species must have a `[species]` root. Uses the common fields.

| Field         | Required | Type         | Description
|---------------|:--------:|--------------|-------------
| `description` | ✓        | String       | A short description for the species.

### Statistic Fields
Fields under the `[species.statistics]` section.

### Moves Fields
Fields under the `[species.moves]`section.

| Field         | Required | Type         | Description
|---------------|:--------:|--------------|-------------
| `learnable`   | ✓        | Script       | The script defining the moves the species can learn.
| `breedable`   |          | String Array | The list of moves that are learnable through breeding with monsters that have these moves.
