movies = {
    "Movie1": ["Action", "Adventure", "Sci-Fi"],
    "Movie2": ["Drama", "Romance"],
    "Movie3": ["Comedy", "Romance"],
    "Movie4": ["Action", "Adventure"],
    "Movie5": ["Comedy", "Family"],
    # Add more movies and their features here
}

# Get user preferences (comma-separated genres)
user_input = input("Enter your preferred movie genres (e.g., Action, Adventure): ")
user_preferences = [genre.strip() for genre in user_input.split(",")]

# Calculate similarity scores between user preferences and movie features
def calculate_similarity(user_prefs, movie_features):
    common_features = set(user_prefs) & set(movie_features)
    return len(common_features) / len(user_prefs)

# Generate movie recommendations for the user
def recommend_movies(user_prefs, movies_data):
    recommendations = []
    for movie, features in movies_data.items():
        similarity_score = calculate_similarity(user_prefs, features)
        recommendations.append((movie, similarity_score))
    recommendations.sort(key=lambda x: x[1], reverse=True)
    return recommendations

# Get movie recommendations for the user
recommended_movies = recommend_movies(user_preferences, movies)

# Print the top 3 recommended movies
print("\nRecommended movies:")
for movie, score in recommended_movies[:3]:
    print(f"{movie} (Similarity Score: {score:.2f})")
