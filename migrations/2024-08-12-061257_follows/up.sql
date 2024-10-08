-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.follows
(
    id bigint NOT NULL GENERATED BY DEFAULT AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 9223372036854775807 CACHE 1 ),
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    following_user_id bigint NOT NULL,
    followed_user_id bigint NOT NULL,
    CONSTRAINT follows_pkey PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.follows
    ADD CONSTRAINT follows_followed_user_id_fkey FOREIGN KEY (followed_user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.follows
    ADD CONSTRAINT follows_following_user_id_fkey FOREIGN KEY (following_user_id)
    REFERENCES public.users (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;