# actix_api
simple API built using rust and actix web framework

endpoints:

/posts
  - GET
    get all the posts
  - POST
    create a new post
    
/posts/:id
  - GET
    get a post by its id
  - DELETE
    delete a post by its id
    
/posts/:id/upvotes
  - GET
    get all the upvotes to a post
  - POST
    add an upvote to a post
  - DELETE
    remove an upvote from the post upvotes
