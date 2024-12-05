create table T (c1 text) strict;
.separator "$"
.import 05_input.txt T
.load ./regex0.so
.mode box
with

-- parse input

-- this table contains all the (pred, succ)
-- rules stating that (pred) should be a
-- predecessor of (succ)
O(r, pred, succ) as (
	select
		T.rowid,
		cast(regex_capture(captures, 1) as integer),
		cast(regex_capture(captures, 2) as integer)
	from
		regex_captures(
			'(\d+)\|(\d+)',
			T.c1
		)
	join T
),

-- this table contains all the integer
-- sequences
--
-- r is the row number, c is the column and
-- v is the value of the column
R(r, c, v) as (
	select
		T.rowid,
		M.rowid,
		cast(match as integer)
	from regex_find_all('\d+', T.c1) as M
	join T
	where T.rowid not in (select r from O)
),

-- this table contains the index of the
-- middle element for a row
M(r, mid) as (
	select
		R.r, 
		count(*)/2
	from R
	group by R.r
),

-- solve

-- for each (row, col) pair we compute
-- how many values on the row should
-- precede the value according to
-- the ordering rules
P(r, c, v, npred) as (
	select
		R1.r,
		R1.c,
		R1.v,
		count(O.pred)
	from
		R as R1
	left join R as R2 on
		R1.r = R2.r
	left join O on
		O.pred = R2.v and
		O.succ = R1.v 
	group by R1.r, R1.v
),

-- this table contains all rows with
-- violations
--
-- i.e. all rows that has a column with an
-- index that is lower than the number of
-- predecessors it should have (pigeonhole
-- principle)
V as (
	select distinct P.r
	from P
	where P.c < P.npred
)

select * from (
	-- part 1
	select
		sum(P.v)
	from P
	join M
	where
		P.r = M.r and
		P.c = M.mid and
		P.r not in V
	union all
	-- part 2
	select sum(P.v)
	from P
	join M
	where
		P.r = M.r and
		-- here we exploit the fact that
		-- the problem description
		-- forces the sequence to be
		-- a totally ordered set, otherwise
		-- the middle element would not be
		-- defined. it follows that the
		-- column index must be equal to
		-- the number of predecessors when
		-- zero indexed
		P.npred = M.mid and
		P.r in V
);
