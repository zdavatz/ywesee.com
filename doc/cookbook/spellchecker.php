<?php if (!defined('PmWiki')) exit(); 
/*  
    This script adds a spell checking functionality to PmWiki
    based on Speller Pages (http://spellerpages.sourceforge.net)
    and GNU Aspell.

    The script was originally prototyped by Ciaran Jessup, and then
    modified by Patrick Michaud (pmichaud@pobox.com) for cleaner
    installation with PmWiki.  

    To use this script, you will need a working copy of GNU aspell
    on your server.  Then, add the following line to your local
    configuration:

        include_once('cookbook/spellchecker.php');

    This will add a "spell check" button to the GUI button bar, and
    link in the necessary spell checking scripts.
*/

# If we're not editing, then just exit.
if ($action != 'edit') return;

# Configure in the Speller Pages javascript code.
$HTMLHeaderFmt[] = "
<script src='$PubDirUrl/speller/spellChecker.js'></script>
<script type='text/javascript' >
function openSpellChecker() {
	var textarea1 = document.getElementById('text');
	var speller = new spellChecker( textarea1 );
	speller.popUpUrl = '$PubDirUrl/speller/spellchecker.html';
	speller.spellCheckScript = '$PubDirUrl/speller/spellchecker.php';
	speller.openChecker();
}
</script>
";

# Add a GUI button for spell checking.
$GUIButtons['spellcheck'] = array(1000, '', '', '',
  "<a href='javascript:openSpellChecker()'><img src='\$GUIButtonDirUrlFmt/spellcheck.gif' title='$[Spell check]' style='border:0px' /></a>");

?>
