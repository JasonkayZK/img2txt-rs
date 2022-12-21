# img2txt-rs

[![CI](https://github.com/JasonkayZK/img2txt-rs/workflows/CI/badge.svg)](https://github.com/JasonkayZK/img2txt-rs/actions)

A cli to generate text image.

<br/>

## **Usage**

It’s easy to use this tool generating text image, all you need to do is provide a path of a image.

For example:

```bash
$ img2txt ./examples/example3.jpg
```

The output:

```
                                                                                                                        
                                                                                                                        
                                                                                                                        
                                                                                                                        
                                                                                :ooo:                                   
                                   .:o:.                                      .:...:ooo                                 
                                   .:o**o.                                   ..   .  .oo.                               
                                      .:*o.      .........................  ... ..:::. :o                               
                                     ....    ..::::...          ...... .......:&####&&: ::                              
                                  .:o*&&*:   ......                       ...::*#@WW@8&: :.                             
                                 o*o&8&***::.                               .:o**8@WW@#&:.:                             
                                o&&#WW@@##&:                                .:oo**&#WW@&* o                             
                               .:8@@W@##@#*:.                               ..:oo**&#W@&*.o.                            
                                *@@@W@@@@8*:                                  ..ooo*&#W8o.o:                            
                               .8@@@W@@#8*.         ....:::............         .:o:*&@#o.o.                            
                               :@@@@@@#&o.       ...:::ooooooooooooooo::....      :::o&@o.o.                            
                            .  o@@@#@#*:      ....:ooooooo::::::::::oooo::::..     :.::8&.o.                            
                            o. :W@##8o.      ....:oo::......::::::::....:ooo::...   ..::*.o                             
                              .:###8:       ...::.. ..:::ooooo********oo::::ooo:..   . ...o                             
                              .:88&:       ..::. .:ooooooooooooooooo******oo::o*o:.      ..                             
                               o&o.       .:.. .oo*oooooooo::::::::::::o**&&&&oo**o:.                                   
                              .:.       .::. .o*oooooo:::::::::::::::::::o*&&&&*o***:.                                  
                             ..        .::  o*oo::::::::::::::::....:::::::o**&&&*o**o.                                 
                             :        .:. .**o:::...:::......      .....:oo::o*&&&*oo*o.                                
                            ..       .:. :&*::.......                    .::o::o*&&*oo&*.                               
                            .       :o. :*o:......                          .:oo:o&&&oo&o              ..:::..          
                                   .o. :*o:....                               .oo:o*&*o*&:          .::... ..:::.       
                                  .o: :*o....                                   :*oo*&*o**.        ::   :oo:.  .:.      
                                 .:: :oo....                                     .ooo&&oo*:       .. .&@WWWW#8:  ..     
                                 :o .*o...                                 .:.     ooo&*oo*.        o@WWWWW@@WW*  o     
                                .o..oo:..      .o&##8*:                  o8@W@8*.   oo*&oo*:       :@W@@@@W@WWW@: ::    
                            .  .:: :o:..     .*#WWWWWW@8.              .8WWWWWWW@*  .oo*&:o*.      &@@@@@@@W@@@@8  :    
                              ..o..o:..     o@WWWWWWWWWW#.             8WWWWWWWWWW8. .oo&oo*:.     #@@@@@@@@@@@8*  :    
                           .  .:: :o..     &WWWWWWWWWWWWW8            oWWWWWWWWWWWW#. :o**:oo..   .@@@@@@#@@@@@&o  :    
                          .. ..o..o:.    .#WWWWWWWWWWWWWWWo           8WWWWW@@WWWWWW#. oo*:o*:.   :@@@@@@@@@@@#&:  :    
                          .. .::.::..   :@WWWWWW&***@WWWWW&           @WWW#***o@WWWWW#..o*o:*o... &@@@@@@@@@@W#&.  o    
                         ..  .o..o..   .@WWWWW@*#88#o#WWWW8          .WWW8&&.8@o@WWWWW& o**:oo..:o8@@@@@@@@@WW#*  .:    
                        ... .:o.:::.  .#WWWWWW**#  @@:@WWW8           @W@*8* *W#&WWWWWWo.**o:*:.o*8#@@@@@@@WW@8.  :.    
                        .:. .o:.::.   &WWWWWW#&#8*&#W&&WWW8           #W8###@@@WoWWWWWW#.**o:*o:**&8@@@@@@WW@@*   o     
                        .. ..o.:::.  :WWWWWWW&8##WW@@#oWWW*           *W&#8WWW@W*@WWWWWWoo***o*:&*&8@@@@@WWWW@.  ::     
                       .:. .:o.:::   #WWWWWWW&##WWW@@@oWW@.   .&&**&: .@&@#WWW#@o@WWWWWW#:*&8o*o&&&8@@WWWWWWWo   :      
                       .:. .:::o::  oWWWWWWWW&###@W#@#*WW&    @@&&&#W: &&#8&###@:WWWWWWWWo**8o*o*&&8@WWWWWWW8   ::      
                      .::. .:::::.  8WWWWWWWW#&#&&88W&&W#    .W@@@@@W* .&&@8&#W**WWWWWWWW&*&8*oo*&&8@WWWWWW#   .o       
                      .:.  .o:::o.  @WWWWWWWWW&#@##W#*WW:     &WWWWWW:  o*&@@@*:@WWWWWWWW8o&8*o*o**8@WWWWW@.   o.       
                      .:.  .o:::o  :WWWWWWWWWWW&8@@8&@W&       &WWW@o    88**o*@WWWWWWWWW#o&8&o*:oo8@WWWW@:   ::        
                      ::.  .o:::o  :WWWWWWWWWWWW@###WW#         :**:.    :WW@WWWWWWWWWWWW@o&&&:*:oo8@WWWWo   .o         
                     .:.   .o:::o  :WWWWWWWWWWWWWWWWW@.           ..      oWWWWWWWWWWWWWW@o*&&:oooo8#@WWo   .o.         
                     .:.   .o:::o  .@WWWWWWWWWWWWWWW@:   ..           ..   o@WWWWWWWWWWWW@o**&:oo:o8#@W*    ::          
                     .:.   .::::o.  8WWWWWWWWWWWWWW@:    .&8*.      :&&:    :@WWWWWWWWWWW@o**&:oo:o8#W*    :o           
                    .::.   .::::o.  *WWWWWWWWWWWWW#.       :#@#8&&8#@&.      .#WWWWWWWWWW#:***:o::o8@*    :o            
                    .::    .:::o:.  .#WWWWWWWWWWW&           :*8###&:          8WWWWWWWWW*:ooo::::o8*    :*.            
                   ....    .:::o::.  .#WWWWWWWW8:               ...             *@WWWWWW#:o*o:::::::    .o:             
                  .....    .:::o::..   *#@W@#&:                                  .&@WWW8::o*o:o:::.    .o:              
                 . .&:     .:::ooo..     ...                                       .:*o:::o*o:::.:.   .oo               
                 . *@o     .::o:o::.                                                 ..:::o*o.::..    :o.               
                  :#W*     .::o:ooo..                                               ...::o**o.::..   ::.                
               . .&@W&  .. ..:oooo:o.                               ..            ....:::*&o:.o..   .:                  
                 o#@W8.... ..:ooooo::.                             ...          ....::::o&*o.:o.   .:                   
                :#WWW8::..  ..::ooo:::                            ....        .....:::::&&o:.oo.   :                    
               .8@WWW#o:..   ..o:ooo:o:     .                    ....        ....::::::*8*:.:*:.                        
               *@WWWW#*:..   ..:::::o:::   .....                ......     ....:::::::*8*:::*o:.                        
              :@WWWWW@&:..   ...o:o::o:::. .......            ...............::::::::o8*o:.o*:..                        
           . .8WWWWW@@8o..    ..:o.::oo:::..........         ..............:::::::::*8*o:.o*o:..                        
             o@WWWW@@@#o..    ...:o::::o::::.. ........   ................::::::..:&&*o:.:*oo:.                         
            .#WWWWW@@@#o..     ...:o.:o:oo:::::.   .......................:::...:*8&*oo::*o:o..                         
         .  *@WWWWW@@@#o..     ....:o::o::o:::ooo::.  .......................:o&8&*ooo:o**o::..                         
           .#WWWWWWW@@@*:..     ....:o:.:o::o:::o***oo:::................:o*&&&**ooo::o**o:o:..                         
        .  *@@WWWWWW@@@*:...     ....::o:.:oo:::oooooo***oooooo::::oooo***&***oooo::o***::o:..                          
          .#@@WWWWWWW##&o:..      .....:oo:::oo::ooooooooooooooooooooooo*o*ooooo:oo*&*o::o::..                          
          o@@@WWWWWW@#8*o:...     .......:ooooo**ooo::::::::::::::::::::oooooooo*&&*o:.:o:::..                          
          8@@WWWWWWW@#&*o::..      .......:::oo********ooooooooooooooooooooo**&&**o:..:o:.::.                           
         :@@WWWWWWWW#8*:.::...      ........::::ooooooooooooo:::::::::::ooo***oo::.::o:..::..                           
         *WWWWWWWWWW8.:..::...         ...:.::o::::::ooooooooooooooooooooooo:::::::::...:::..                           
      .. &WWWWWWWWWW&   ..:...           ...::o::::::::::::::::::::::::::::::::::::....::::.                            
      .. 8WWWWWWWWWWo.    .....            ...::::::::::::::::::::::::::::::::::.........:..                            
      o: &@WWWWWWWW@:.    ......              .....::::::::::::..........................:..                            
      :: *#@WWWWWWW# .     ......                  .......:...............    ..........:..                             
      .o.:##WWWWWWWo .     .......                                             .........:..                             
       :: o##@WWWW* .      .......                                             ........:..                              
       .o..:&8##8: :       ........                          *8*.              ........:..                              
        .o:..... .:.       .........                         *#8o              ..........                               
          oo:..:::          ........                        .oo::..            .........                                
           .:o::.           .........                       o*o*o:..           ........                                 
                             .........                     **:...             .........                                 
                              .........                    . .o:             .........                                  
                              ...........                    oo             ..........                                  
                              ............                   .&*.          .......:..                                   
                              . ............                  .&o       ........::..                                    
                              ..................         .         ... ........::...                                    
                               . ..................     :8&*&&&o:&*&**.............                                     
                               . ................... .. :*o****o:*o***............                                      
                               .. ......................   .:..:..:.  ...........                                       
                               .: ::.......................:.:o.o:.:............. ..                                    
                                : :&o:.....................:.:o:*o::..........:..:o.                                    
                                ..:&&&*:....................:.:o::o:........:::o&&:.                                    
                                ...&&&&&*o:.....................::.....:.::oo*&88&..                                    
                                .: *&&&&&&&*:.....:..............::::::::o**&&888*..                                    
                                .o *8&&&&&&&&*o.........::::.::::::::::o***&8888#*.                                     
                                 o *88888&&&&&&*:....:::::::::::::::::o&&&&888###o.                                     
                                 o.o88888888&&&&*o::::::::::::::::::o*&&&88######o.                                     
                                 o.o#888888888&&**oo:::::::::::::::*&8888##@@@@@#:.                                     
                                 o.*##########88&***oooo::::o:::o*&88##@@@@@@@@@#:.                                     
                                 o:*###@@@@@@@@@##8&*:.      .:&8##@@@@@@@W@WW@@@:.                                     
                                .o:*@@@@@@@@@WW@@@#&*         .&@@@WWWWWWWWWW@@@#o.                                     
                                .o:o@@@@@@@@W@@@@@#oo         .o#@@@WWWWWWWW@@@@8o:                                     
                                .oo:#@@@@@@WW@@@W@#.            8WWWWWWWWWWW@WW@&oo                                     
                                .*oo#W@W@WWWW@@@W@@:            8WWWWWWWWWWWWWW#&*o                                     
                                .ooo8WWWWWWWWWWW@WW&           .8WWWWWWWWWWWWWW8**o                                     
                                .::o*#WWWWWWWWWWWWW&           .8WWWWWWWWWWWWW#&*::                                     
                                .o:ooo*&8##@@@@@#8&o.          :*#@WWWWWWW@@#&&*o::                                     
                                 . ..:oo::oooo*oooo.            ooo***&***oooo:..::                                     
                                     .:ooo::::o::.               :ooooo:::ooo:..::                                      
                                   ....o*o.                            .  .o*o:::                                       
                                              ..                   .                                                    
                                                                                                                        
                                                                                                                        
```

And a size can be given to determine the output size:

```bash
$ img2txt -s 20 ./examples/example3.jpg
```

The output:

```
                    
     .o      *o     
     &8. ....o&     
     o..::::o:.     
      :.    .oo  :. 
     ..*#o .#&o.oW8 
    ..&@#8 o#@8o8W* 
    .:@W@:oo&WW*8#. 
    .:8@o :..#@oo.  
  .*.:.       o:.   
  &# ::.  ...:o:    
 .W#. :oo:::ooo.    
 *W*.  .:::::::     
 :*  .    :  ..     
     ..   :  .      
     .:...:...      
     .&*:::o&*      
     :@W& oWW&      
     .*&o .8&:      

```

<br/>

## **Install**

It’s easy to just use cargo install this tool:

```bash
$ cargo install img2txt-rs
```

<br/>
