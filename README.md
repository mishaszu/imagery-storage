# Imagery Storage
Imagery Storage

## Tech Stack:
- Axum - BE framework
- Diesel - PostgreSQL ORM
- async-graphql - GraphQL library
- request - for proxing requests to Lust

## Design:
user -> imagery -> lust
User makes a request to imagery, if requesting an image, imagery fetches it from Lust and sends it back to the user.

## Access schema:
Roles:
### Admin
- can view and moderate all users, posts, albums & images
### Creator
- can create posts, albums and upload images
- can comment own posts
- can moderate own posts comments
### Commenter
- can comment post of subscriber
- can edit and delete owned comments
### Guest
- can view public and subscriber posts

## Access levels:
### 1. No user access to public creator
- can view public user profile
- can view public posts
- can view public albums
- can't see public posts comments
- can't comment on public posts
- can't see private posts & albums
- can view subscriber lvl posts as null (UI can display placeholder with "Content only for subscribers")
### 2. User no subscriber to public & subscriber creator
- the same as 1. but can view comments on public posts of public user
### 3. User subscriber to public & subscriber creator
- the same as 1. with:
- can see public comments of public user
- can see subscriber lvl posts & albums of public user
- can see subscriber lvl posts comments of public user
### 4. No user access to subscriber creator
- can view public user profile
- can't view public posts
- can't view public albums
- can't see public posts comments
- can't comment on public posts
- can't see private posts & albums
- can view subscriber lvl posts as null (UI can display placeholder with "Content only for subscribers")
- can view public lvl posts as null (UI can display placeholder with "Content only for subscribers")
### 5. Any user to private creator
- can't see anything - should return 404

## Resource Access level checks:
### Account
- check if logged account is admin
### User
- check if logged account is admin
- check if logged account is bocked
- check if logged account is owner
- check if user is blocked
- check if user is private
### Post
- check if logged account is admin
- check if logged account is bocked
- check if logged account is owner
- check if post user is blocked
- check if post user is private
- check if post is private
- check if post is on feed [for getting user posts]
- check if sub user account has referral for post user account [for getting user & album posts]
- check if post user account & post are public [for getting user posts]
- check if post user account & album & post are public [for getting album posts]
### Image
- check if no user has access to image post
- check if user has access to image post
### Comment
- check if no user has access to comment post
- check if user has access to comment post
- check if post allows comments
### Fav
- check if no user has access to fav post
- check if user has access to fav post
### Tag
- check if no user has access to tag post
- check if user has access to tag post
