CREATE TABLE Recipe (
    PRIMARY KEY id AUTO_INCREMENT,
    name: string,
    description: string,
    instructions: string,
    ingredients: string[],
    views: int,
    ratings: int,
    total_rating: int,
    cousine_id: int
);

CREATE TABLE RecipeType (
    id AUTO_INCREMENT,
    recipe_id: int,
    FOREIGN KEY recipe_id REFERENCES Recipe(id),
    PRIMARY KEY (id, recipe_id),
    meal_id: int
);  

CREATE TABLE RecipeIngredient (
    ingredient_id,
    recipe_id: int,
    FOREIGN KEY recipe_id REFERENCES Recipe(id),
    PRIMARY KEY (ingredient_id, recipe_id),
    ingredient_id: int,
    ingredient_count: int,
    amount: int,
    -- For example: "for garnish"
    description: string 
);

-- Map many ingredients to one name
CREATE TABLE IngredientName (
    PRIMARY KEY id AUTO_INCREMENT,
    name: string
);

CREATE TABLE RecipeCousine (
    id AUTO_INCREMENT,
    recipe_id: int,
    FOREIGN KEY recipe_id REFERENCES Recipe(id),
    PRIMARY KEY (id, recipe_id),
    cousine_id: int,
);

CREATE TABLE Comment (
    user_id int,
    FOREIGN KEY user_id,
    recipe_id: int,
    FOREIGN KEY recipe_id REFERENCES Recipe(id),
    PRIMARY KEY (user_id, recipe_id),
    comment: string,
    rating: int
);

CREATE TABLE RecipeCollection (
    id AUTO_INCREMENT,
    user_id: int,
    FOREIGN KEY user_id, REFERENCES User(id),
    PRIMARY KEY (id, user_id),
    collection_name: string
);

-- Map many recipes to one collection
CREATE TABLE RecipeCollectionRecipe (
    recipe_id: int,
    collection_id: int,
    PRIMARY KEY (recipe_id, collection_id),
    FOREIGN KEY collection_id REFERENCES RecipeCollection(id),
    FOREIGN KEY recipe_id REFERENCES Recipe(id),
);

CREATE TABLE User (
    PRIMARY KEY id AUTO_INCREMENT,
    username: string,
    email: string,
    password: string
);