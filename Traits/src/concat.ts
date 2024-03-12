type UserProfile = {
  name: string;
  age: number;
};
// Create a Map to store user profiles
const profiles = new Map<string, UserProfile>();
// Add a user profile
profiles.set("user1", { name: "Alice", age: 30 });

// Later, you can add more user profiles without having to update a type definition
profiles.set("user2", { name: "Bob", age: 25 });
profiles.get('user1')