// @generated automatically by Diesel CLI.

diesel::table! {
    book (book_id) {
        book_id -> Nullable<Integer>,
        book_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    course (course_id) {
        course_id -> Nullable<Integer>,
        course_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    ingredient (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    recipe (recipe_id) {
        recipe_id -> Nullable<Integer>,
        primary_season -> Integer,
        course_id -> Integer,
        book_id -> Nullable<Integer>,
        recipe_name -> Nullable<Text>,
        recipe_url -> Nullable<Text>,
        created_at -> Nullable<Float>,
        page -> Nullable<Integer>,
    }
}

diesel::table! {
    recipe_ingredient (recipe_id, ingredient_id) {
        recipe_id -> Integer,
        ingredient_id -> Integer,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    season (season_id) {
        season_id -> Nullable<Integer>,
        tag_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    tried (user_id, recipe_id) {
        user_id -> Integer,
        recipe_id -> Integer,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    user (id) {
        id -> Nullable<Integer>,
        email -> Text,
        pw_hash -> Text,
        created_at -> Nullable<Float>,
    }
}

diesel::joinable!(recipe -> book (book_id));
diesel::joinable!(recipe -> course (course_id));
diesel::joinable!(recipe -> season (primary_season));
diesel::joinable!(recipe_ingredient -> ingredient (recipe_id));
diesel::joinable!(tried -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    book,
    course,
    ingredient,
    recipe,
    recipe_ingredient,
    season,
    tried,
    user,
);
