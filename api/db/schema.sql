CREATE TABLE User (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE Recipe (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    instructions TEXT,
    ingredients TEXT[],
    views INTEGER DEFAULT 0,
    ratings INTEGER DEFAULT 0,
    total_rating INTEGER DEFAULT 0,
    cousine_id INTEGER
);

CREATE TABLE RecipeType (
    id SERIAL,
    recipe_id INTEGER REFERENCES Recipe(id),
    meal_id INTEGER NOT NULL,
    PRIMARY KEY (id, recipe_id)
);  

CREATE TABLE RecipeIngredient (
    recipe_id INTEGER REFERENCES Recipe(id),
    ingredient_id INTEGER NOT NULL,
    ingredient_count INTEGER,
    amount INTEGER,
    description VARCHAR(255),
    PRIMARY KEY (ingredient_id, recipe_id)
);

-- Map many ingredients to one name
CREATE TABLE IngredientName (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE RecipeCousine (
    id SERIAL,
    recipe_id INTEGER REFERENCES Recipe(id),
    cousine_id INTEGER NOT NULL,
    PRIMARY KEY (id, recipe_id)
);

CREATE TABLE Comment (
    user_id INTEGER REFERENCES User(id),
    recipe_id INTEGER REFERENCES Recipe(id),
    comment TEXT,
    rating INTEGER,
    PRIMARY KEY (user_id, recipe_id)
);

CREATE TABLE RecipeCollection (
    id SERIAL,
    user_id INTEGER REFERENCES User(id),
    collection_name VARCHAR(255) NOT NULL,
    PRIMARY KEY (id, user_id)
);

-- Map many recipes to one collection
CREATE TABLE RecipeCollectionRecipe (
    recipe_id INTEGER REFERENCES Recipe(id),
    collection_id INTEGER REFERENCES RecipeCollection(id),
    PRIMARY KEY (recipe_id, collection_id)
);


-- CREATE TABLE Recipe (
--     PRIMARY KEY id AUTO_INCREMENT,
--     name: string,
--     description: string,
--     instructions: string,
--     ingredients: string[],
--     views: int,
--     ratings: int,
--     total_rating: int,
--     cousine_id: int
-- );

-- CREATE TABLE RecipeType (
--     id AUTO_INCREMENT,
--     recipe_id: int,
--     FOREIGN KEY recipe_id REFERENCES Recipe(id),
--     PRIMARY KEY (id, recipe_id),
--     meal_id: int
-- );  

-- CREATE TABLE RecipeIngredient (
--     ingredient_id,
--     recipe_id: int,
--     FOREIGN KEY recipe_id REFERENCES Recipe(id),
--     PRIMARY KEY (ingredient_id, recipe_id),
--     ingredient_id: int,
--     ingredient_count: int,
--     amount: int,
--     -- For example: "for garnish"
--     description: string 
-- );

-- -- Map many ingredients to one name
-- CREATE TABLE IngredientName (
--     PRIMARY KEY id AUTO_INCREMENT,
--     name: string
-- );

-- CREATE TABLE RecipeCousine (
--     id AUTO_INCREMENT,
--     recipe_id: int,
--     FOREIGN KEY recipe_id REFERENCES Recipe(id),
--     PRIMARY KEY (id, recipe_id),
--     cousine_id: int,
-- );

-- CREATE TABLE Comment (
--     user_id int,
--     FOREIGN KEY user_id,
--     recipe_id: int,
--     FOREIGN KEY recipe_id REFERENCES Recipe(id),
--     PRIMARY KEY (user_id, recipe_id),
--     comment: string,
--     rating: int
-- );

-- CREATE TABLE RecipeCollection (
--     id AUTO_INCREMENT,
--     user_id: int,
--     FOREIGN KEY user_id, REFERENCES User(id),
--     PRIMARY KEY (id, user_id),
--     collection_name: string
-- );

-- -- Map many recipes to one collection
-- CREATE TABLE RecipeCollectionRecipe (
--     recipe_id: int,
--     collection_id: int,
--     PRIMARY KEY (recipe_id, collection_id),
--     FOREIGN KEY collection_id REFERENCES RecipeCollection(id),
--     FOREIGN KEY recipe_id REFERENCES Recipe(id),
-- );

-- CREATE TABLE User (
--     PRIMARY KEY id AUTO_INCREMENT,
--     username: string,
--     email: string,
--     password: string
-- );