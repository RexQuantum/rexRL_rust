#[derive(PartialEq, Copy, Clone)]
pub struct PrefabLevel {
    pub template : &'static str,
    pub width : usize,
    pub height: usize
}

#[allow(dead_code)]
pub const WFC_POPULATED : PrefabLevel = PrefabLevel{
    template : LEVEL_MAP,
    width: 80,
    height: 43
};

#[allow(dead_code)]
const LEVEL_MAP : &str =
"
################################################################################
#          ########################################################    #########
#    @     ######    #########       ####     ###################        #######
#          ####   C  #                          ###############            #####
#          #### #    # #######       ####       #############                ###
##### ######### #    # #######       #########  ####    #####                ###
##### ######### ###### #######   C   #########  #### ## #####                ###
##                        ####       #########   ### ##         C            ###
##### ######### ###       ####       #######         ## #####                ###
##### ######### #####  ## ####       ####### #   ### ## #####                ###
##### ######### #####     ####       ####### #######    #####     C          ###
###          ## #####  ## ####       ####### ######### ######                ###
###          ## ##### C## ###### ########### #   ##### ######                ###
###          ## ##### C## ###### ###########     ###                         ###
###    %                  ###### ########### #   ###   !   ##                ###
###          ## ####   ##        ######   ## #######       ##                ###
###          ## #####     ## ### #####     # ################## #####      #####
###          ## ######    ## ### #####     # #   ############## #######    #####
###      ###### ###### ##### ### ####          C ######             ###    #####
####    ####### #####  ####   ## ####        #   ######             ##### ######
#    #  ####### ####   ####   ## ####        ##########             ##### ######
# C  ##  ###### ###    ####   ##        %    ##########           C  #### #    #
#    ##                ####   ## ####        #   ######     #######  ####   C  #
#######                  ####### ####                       !    !    ### #    #
######                     ##### ####        #   ######               ### ######
#####                            #####     # ##########     ##  ##    ### ######
#####           !           ### ######     # ##########     #C  C#    ### #   ##
#####                       ### #######   ## #   ######     ######    ###   C ##
#   ##                     #### ######## ###   C #######  ^        ^ #### #   ##
# C    #                 ###### ######## #####   #######  ^        ^ #### ######
#   ##C####           ######    ######## ################           ##### ######
#   ## ########## ##########    ######## #################         ######      #
#####   ######### ########## %  ######## ###################     ######## ##   #
#### # # ######## ##########    ######## #################### ##########   #   #
### ## ## ######   #########    ########          ########### #######   # C#   #
### ## ##           ###############      ###      ########### #######   ####   #
### ## ## ####       ############## ########    C ########### ####         # ^ #
#### # #^####         ############# ########      #####       ####      # C#   #
#####   ######       ###            ########      ##### C     ####   !  ####^^ #
#!%^## ###  ##           ###################  CC                 C         # > #
#!%^   ###  ###     ########################  C   ##### C     ####      # C#   #
# %^##  ^   ###     ########################      #####       ##################
################################################################################
";