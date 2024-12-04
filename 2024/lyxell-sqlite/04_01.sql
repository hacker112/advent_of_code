create table T (c1 text) strict;
.import 04_input.txt T
.load ./regex0.so
.mode box

-- parse input
create table MAT (row integer, col integer, value text) strict;
insert into MAT
select
	T.rowid - 1,
	start,
	match
from regex_find_all("(.)", T.c1)
join T;
create table X (row integer, col integer) strict;
create table M (row integer, col integer) strict;
create table A (row integer, col integer) strict;
create table S (row integer, col integer) strict;
insert into X select row, col from MAT where value = "X";
insert into M select row, col from MAT where value = "M";
insert into A select row, col from MAT where value = "A";
insert into S select row, col from MAT where value = "S";

-- solve
with

C(c) as (
	select * from generate_series(0, 140)
),

R(r) as (
	select * from generate_series(0, 140)
)

select count(*) from (
	-- horizontal/vertical
	select *
		from X
		join M
		join A
		join S
		join C
		join R
		join (values (1), (-1)) as D
		join (values (0), (1)) as E
	where (
		X.col = c and
		M.col = c + 1 * D.column1 * E.column1 and
		A.col = c + 2 * D.column1 * E.column1 and
		S.col = c + 3 * D.column1 * E.column1 and
		--
		X.row = r and
		M.row = r + 1 * D.column1 * (1 - E.column1) and
		A.row = r + 2 * D.column1 * (1 - E.column1) and
		S.row = r + 3 * D.column1 * (1 - E.column1)
	)
	union all
	-- diagonal
	select *
		from X
		join M
		join A
		join S
		join C
		join R
		join (values (1), (-1)) as D1
		join (values (1), (-1)) as D2
	where (
		X.row = c and
		M.row = c - 1 * D1.column1 and
		A.row = c - 2 * D1.column1 and
		S.row = c - 3 * D1.column1 and
		--
		X.col = r and
		M.col = r + 1 * D2.column1 and
		A.col = r + 2 * D2.column1 and
		S.col = r + 3 * D2.column1
	)
);
