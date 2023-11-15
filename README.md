# Imagery Storage
Imagery Storage

## Tech Stack:
- Axum
- Diesel
- async-graphql
- request

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
- check if user is blocked
- check if user is private
### Post
- check if logged account is admin
- check if logged account is bocked
- check if post user is blocked
- check if post user is private
- check if post is private
- check if sub user account has referral for post user account
- check if post user account & post are public
### Image
- check if logged account is admin
- check if logged account is bocked
- check if image post user is blocked
- check if image post user is private
- check if image post is private
- check if image sub user account has referral for post user account
- check if image post user account & post are public

## Access tree
### idea:
- add optional no graphql field to all graphql models with allowed access level for easier access propagation
- add access to all get methods:
    - if private and can't access return error
    - if sub and no access return None
- add access to all list methods to:
    - to filter out all private if no access
    - to return None for all sub or pub no access
- all access checks should return structure to reduce DB calls
    - if checking for user access should return user if has access
    - if checking for album access should return album if has access
    - if checking for post access should return post if has access
    - if checking for image access should return image if has access
- only admin / owner queries and mutations should have guards

### flow: 
- [admin] account -> user
- [admin] user -> account
-  user -> album -> posts -> image
-  user -> posts[feed] -> image
-  posts[all feeds] -> image
-  post -> image


