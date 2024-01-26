--
-- PostgreSQL database dump
--

-- Dumped from database version 16.1
-- Dumped by pg_dump version 16.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration text NOT NULL,
    data_price text NOT NULL
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2 DAYS	200 NAIRA
2	1.8GB	14 DAYS	500 NAIRA
3	3.9GB	30 DAYS	1000 NAIRA
4	7.5GB	30 DAYS	1500 NAIRA
5	9.2GB	30 DAYS	2000 NAIRA
6	10.8GB	30 DAYS	2500 NAIRA
7	14GB	30 DAYS	3000 NAIRA
8	18GB	30 DAYS	4000 NAIRA
9	24GB	30 DAYS	5000 NAIRA
10	29.9GB	30 DAYS	8000 NAIRA
11	50GB	30 DAYS	10000 NAIRA
\.


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- PostgreSQL database dump complete
--

