CREATE TABLE IF NOT EXISTS User (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS Recipe (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    instructions TEXT NOT NULL,
    summary VARCHAR(255) NOT NULL,
    ratings INTEGER DEFAULT 0 NOT NULL,
    views INTEGER DEFAULT 0,
    total_rating INTEGER DEFAULT 0 NOT NULL,
    -- The URL where the recipe came from. May not have a source
    source TEXT,
    -- Some recipes don't have an author - they are auto-generated
    author_id INTEGER REFERENCES User(id),
    public BOOLEAN DEFAULT TRUE
);

CREATE TABLE IF NOT EXISTS RecipeMeal (
    recipe_id INTEGER REFERENCES Recipe(id) NOT NULL,
    meal_id INTEGER NOT NULL REFERENCES MealName(id),
    PRIMARY KEY (recipe_id, meal_id)
);  

CREATE TABLE IF NOT EXISTS MealName (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS RecipeIngredient (
    recipe_id INTEGER REFERENCES Recipe(id) NOT NULL,
    ingredient_id INTEGER NOT NULL REFERENCES IngredientName(id),
    amount REAL NOT NULL,
    description VARCHAR(255) NOT NULL,
    PRIMARY KEY (ingredient_id, recipe_id)
);

-- Map many ingredients to one name
CREATE TABLE IF NOT EXISTS IngredientName (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    affiliate_link VARCHAR(255),
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS RecipeCuisine (
    recipe_id INTEGER NOT NULL REFERENCES Recipe(id),
    cuisine_id INTEGER NOT NULL REFERENCES CuisineName(id),
    PRIMARY KEY (recipe_id, cuisine_id)
);

CREATE TABLE IF NOT EXISTS CuisineName (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS Comment (
    user_id INTEGER NOT NULL REFERENCES User(id),
    recipe_id INTEGER NOT NULL REFERENCES Recipe(id),
    comment TEXT NOT NULL,
    rating INTEGER NOT NULL,
    PRIMARY KEY (user_id, recipe_id)
);

CREATE TABLE IF NOT EXISTS RecipeCollection (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER REFERENCES User(id),
    collection_name VARCHAR(255) NOT NULL
);

-- Map many recipes to one collection
CREATE TABLE IF NOT EXISTS RecipeCollectionRecipe (
    public BOOLEAN DEFAULT TRUE,
    recipe_id INTEGER NOT NULL REFERENCES Recipe(id),
    collection_id INTEGER NOT NULL REFERENCES RecipeCollection(id),
    PRIMARY KEY (recipe_id, collection_id)
);

CREATE TABLE IF NOT EXISTS RecipeDiet (
    recipe_id INTEGER NOT NULL REFERENCES Recipe(id),
    diet_id INTEGER NOT NULL REFERENCES DietName(id),
    PRIMARY KEY (recipe_id, diet_id)
);

CREATE TABLE IF NOT EXISTS DietName (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS LoginToken (
    user_id INTEGER PRIMARY KEY NOT NULL REFERENCES User(id),
    token VARCHAR(255) NOT NULL,
    created_epoch INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS RecipeSource (
    recipe_id INTEGER NOT NULL REFERENCES Recipe(id),
    source_url VARCHAR(255) NOT NULL,
    source_id INTEGER NOT NULL REFERENCES Source(id),
    PRIMARY KEY (recipe_id, source_id)
);

CREATE TABLE IF NOT EXISTS Source (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    home_url VARCHAR(255) NOT NULL,
    -- How good of a source we consider them to be. This should be an average, so if they have some good recipes and some bad, their score should be like 2.5. If they only have good recipes, this should be 5
    rating INTEGER NOT NULL
);