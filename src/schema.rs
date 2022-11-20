// @generated automatically by Diesel CLI.

diesel::table! {
    Book (book_id) {
        book_id -> Nullable<Integer>,
        book_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    Course (course_id) {
        course_id -> Nullable<Integer>,
        course_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::table! {
    Recipe (recipe_id) {
        recipe_id -> Nullable<Integer>,
        primary_season -> Integer,
        course -> Integer,
        book -> Nullable<Integer>,
        recipe_name -> Nullable<Text>,
        recipe_url -> Nullable<Text>,
        created_at -> Nullable<Float>,
        page -> Nullable<Integer>,
    }
}

diesel::table! {
    Season (season_id) {
        season_id -> Nullable<Integer>,
        tag_name -> Nullable<Text>,
        created_at -> Nullable<Float>,
    }
}

diesel::joinable!(Recipe -> Book (book));
diesel::joinable!(Recipe -> Course (course));
diesel::joinable!(Recipe -> Season (primary_season));

diesel::allow_tables_to_appear_in_same_query!(
    Book,
    Course,
    Recipe,
    Season,
);
