# User Auth

## Questions

- [ ] Is it okay to use *signed* UUIDv7 for session IDs?
- [ ] How can I return two cookie jars at once? Just use two layers?

## Tasks

### Invite User (Admin)
- [x] Create user record
- [x] Set OTP for user
- [x] Encode OTP in URL-safe manner
- [x] Send email invite to user

### Edit User (Admin)
- [x] Find user by ID
- [ ] Update fields provided

### Log In
- [x] Validate user exists
- [x] Validate password hash
- [x] Create session in DB
- [x] Store session cookie

### Log Out
- [x] Delete Session from DB
- [x] Delete Session Cookie

### Reset Password
- [x] Ensure OTP isn't too old
- [ ] Set password

### Activate Account
- [x] Ensure OTP isn't too old
- [ ] Set password and display name
- [ ] Redirect to login (or create session?)

### Update Account
- [ ] Find self by ID
- [ ] Update fields provided

### Validate Session (Backend)
- [x] Get cookie
- [x] Ensure session exists in DB
- [x] Ensure session is not too old
- [x] Ensure user exists and is enabled
- [x] Trigger "log out" if session is too old or otherwise
