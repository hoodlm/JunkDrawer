This is a toybox for working with the text of the [Voynich manuscript](https://en.wikipedia.org/wiki/Voynich_manuscript).

Run `pull.bash` to retrieve transcriptions/data for the tool to run. (It currently only retrieves the 'FSG' transcription.)

The main program is hard-coded to naively choose the lefthand side of any either-or in the transcription.
It randomly maps the 'words' of the manuscript to 950 'simple english' words. The manuscript has
about ~3000 unique words so there will be some synonyms.

Here's an example 'translation' of the first three paragraphs:

```
Well art strong line cheese please tin nose certain
peace stop flower get living motion strong town town sponge 
moon sound narrow bucket learning egg linen shut sleep 
across carriage insect shirt dust powder week narrow need 
muscle sound steel poison mist.

True. 

Death ice cheese vessel surprise rough war sweet bucket 
limit be sister deep arm organization company egg ant rate 
powder motion put be.
```

This software is GPLv3 licensed; see [COPYING](./COPYING).
