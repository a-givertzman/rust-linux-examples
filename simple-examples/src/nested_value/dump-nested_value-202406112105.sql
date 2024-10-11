--
-- PostgreSQL database dump
--

-- Dumped from database version 13.14 (Debian 13.14-0+deb11u1)
-- Dumped by pg_dump version 13.14 (Debian 13.14-0+deb11u1)

-- Started on 2024-06-11 21:05:59 MSK

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

--
-- TOC entry 3006 (class 1262 OID 65941)
-- Name: nested_value; Type: DATABASE; Schema: -; Owner: postgres
--

CREATE DATABASE nested_value WITH TEMPLATE = template0 ENCODING = 'UTF8' LOCALE = 'en_US.UTF-8';


ALTER DATABASE nested_value OWNER TO postgres;

\connect nested_value

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

--
-- TOC entry 3 (class 2615 OID 2200)
-- Name: public; Type: SCHEMA; Schema: -; Owner: postgres
--

CREATE SCHEMA public;


ALTER SCHEMA public OWNER TO postgres;

--
-- TOC entry 3007 (class 0 OID 0)
-- Dependencies: 3
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA public IS 'standard public schema';


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 203 (class 1259 OID 65952)
-- Name: array_test; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.array_test (
    id integer NOT NULL,
    value integer NOT NULL
);


ALTER TABLE public.array_test OWNER TO postgres;

--
-- TOC entry 202 (class 1259 OID 65950)
-- Name: array_test_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.array_test_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.array_test_id_seq OWNER TO postgres;

--
-- TOC entry 3008 (class 0 OID 0)
-- Dependencies: 202
-- Name: array_test_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.array_test_id_seq OWNED BY public.array_test.id;


--
-- TOC entry 201 (class 1259 OID 65944)
-- Name: map_test; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.map_test (
    id integer NOT NULL,
    key character varying(255) NOT NULL,
    value numeric(16,8) NOT NULL
);


ALTER TABLE public.map_test OWNER TO postgres;

--
-- TOC entry 200 (class 1259 OID 65942)
-- Name: map_test_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.map_test_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.map_test_id_seq OWNER TO postgres;

--
-- TOC entry 3009 (class 0 OID 0)
-- Dependencies: 200
-- Name: map_test_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.map_test_id_seq OWNED BY public.map_test.id;


--
-- TOC entry 2862 (class 2604 OID 65955)
-- Name: array_test id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.array_test ALTER COLUMN id SET DEFAULT nextval('public.array_test_id_seq'::regclass);


--
-- TOC entry 2861 (class 2604 OID 65947)
-- Name: map_test id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.map_test ALTER COLUMN id SET DEFAULT nextval('public.map_test_id_seq'::regclass);


--
-- TOC entry 3000 (class 0 OID 65952)
-- Dependencies: 203
-- Data for Name: array_test; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.array_test VALUES (1, 123);
INSERT INTO public.array_test VALUES (2, 456);
INSERT INTO public.array_test VALUES (3, 789);


--
-- TOC entry 2998 (class 0 OID 65944)
-- Dependencies: 201
-- Data for Name: map_test; Type: TABLE DATA; Schema: public; Owner: postgres
--

INSERT INTO public.map_test VALUES (1, 'key1', 111.11100000);
INSERT INTO public.map_test VALUES (2, 'key2', 222.22200000);
INSERT INTO public.map_test VALUES (3, 'key3', 333.33300000);


--
-- TOC entry 3010 (class 0 OID 0)
-- Dependencies: 202
-- Name: array_test_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.array_test_id_seq', 3, true);


--
-- TOC entry 3011 (class 0 OID 0)
-- Dependencies: 200
-- Name: map_test_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.map_test_id_seq', 3, true);


--
-- TOC entry 2866 (class 2606 OID 65957)
-- Name: array_test array_test_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.array_test
    ADD CONSTRAINT array_test_pkey PRIMARY KEY (id);


--
-- TOC entry 2864 (class 2606 OID 65949)
-- Name: map_test map_test_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.map_test
    ADD CONSTRAINT map_test_pkey PRIMARY KEY (id);


-- Completed on 2024-06-11 21:05:59 MSK

--
-- PostgreSQL database dump complete
--

