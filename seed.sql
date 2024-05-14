-- test@test.com / password
INSERT INTO users (id, email, display_name, enabled, created, secret)
VALUES ('018f63b5-2d12-74cd-8e2a-18e53a008852', 'test@test.com', 'Test User', true, NOW(),
        '$argon2id$v=19$m=65536,t=3,p=1$cGFzc3dvcmQ$GVqOSQmMgkryNpvUA/2aPg');
