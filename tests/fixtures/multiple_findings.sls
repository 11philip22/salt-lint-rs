testfile:
  file.managed:
    - source: /salt://lorem/ipsum/dolor/sit/amet/consectetur/adipiscing/elit/sed/do/eiusmod/tempor/incididunt/ut/labore/et/dolore/magna/aliqua/ut/enim/ad/minim/veniam/really/long/path/for/testing  
    - requires:
        - otherfile
