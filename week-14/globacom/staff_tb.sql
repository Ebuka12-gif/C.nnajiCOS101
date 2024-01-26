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
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    staff_sal integer NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
102	Mustapha Ali	3	175000	32	08063285831
107	Alokwe Martin	7	378000	48	0709008212
97	Dankade Aminat	5	550000	40	09023688832
108	Josiah Joshua	1	120000	30	0805318131
100	Mankinde Jane	2	450000	55	0902348730
120	Adeleke Jane	4	200000	38	0706104862
122	Osahon Mark	6	320000	44	0802228911
117	Suleman Ajayi	3	800000	50	07030089811
104	Kuti Lawal	1	750000	35	09145689842
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

