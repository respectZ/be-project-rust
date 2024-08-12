-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.company_position
(
    id bigint NOT NULL GENERATED BY DEFAULT AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 9223372036854775807 CACHE 1 ),
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    position_id bigint NOT NULL,
    company_id bigint NOT NULL,
    CONSTRAINT company_position_pkey PRIMARY KEY (id),
    CONSTRAINT company_position_unique_company_position UNIQUE (company_id, position_id)
);

ALTER TABLE IF EXISTS public.company_position
    ADD CONSTRAINT company_position_company_id_fkey FOREIGN KEY (company_id)
    REFERENCES public.company (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;


ALTER TABLE IF EXISTS public.company_position
    ADD CONSTRAINT company_position_position_id_fkey FOREIGN KEY (position_id)
    REFERENCES public."position" (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;

ALTER TABLE IF EXISTS public.users
    ADD CONSTRAINT company_position_users_id_fkey FOREIGN KEY (company_position_id)
    REFERENCES public.company_position (id) MATCH SIMPLE
    ON UPDATE NO ACTION
    ON DELETE NO ACTION;