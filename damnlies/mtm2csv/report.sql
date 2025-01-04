.mode csv
.headers on
.import final.csv mtm

select strftime("%Y", date) as year,count(1) as total_songs from mtm group by year;


select submitter,count(1) as total_songs from mtm group by submitter order by total_songs desc;


create table artist_counts as select artist,count(1) as total_songs from mtm group by artist;

select * from artist_counts where total_songs >= 4 order by total_songs desc;


create table dupes as select title,artist,count(1) as occurrences from mtm group by title, artist order by occurrences desc;

select count(1) as heard_at_least_twice from dupes where occurrences > 1;

select * from dupes where occurrences > 2;

