<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_var main  parentvar myPos  parentvar n_a0ec78</name>
   <tag></tag>
   <elementGuidId>b6418eee-952d-4177-b0b7-bbfedb8f7bca</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
      <webElementGuid>d64d6883-6605-48fc-b1c8-650eb12f9718</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>dir</name>
      <type>Main</type>
      <value>ltr</value>
      <webElementGuid>0ace8d2c-112f-4c35-8c27-f2d95f214c58</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>pt_PT</value>
      <webElementGuid>3f745c9a-4e7f-44f1-b244-21a05f6746ae</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>device</name>
      <type>Main</type>
      <value>oraDesktop</value>
      <webElementGuid>ddf2faa1-754a-4dea-9143-f8db16349c9e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>




var main = parent;
var myPos = parent;
var noOfZones = 1;
 var refreshPeriodList=[]; var zoneIntervals=[]; refreshPeriodList[1]='0';var portalOptions={&quot;lists&quot;:{&quot;optionValues&quot;:{&quot;name&quot;:&quot;optionValues&quot;,&quot;header&quot;:{&quot;moreRows&quot;:false,&quot;remainingRows&quot;:0,&quot;lastInfo&quot;:&quot;&quot;},&quot;list&quot;:[]}}};var portalMOAndKeys = {};pushMOAndKeysIfAvailable();
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

main.pendingGlobalContext.waitForZone(0);



Pesquisa de Formulários Fiscais

 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField['EMPTY_SUMMARY']={ fieldValue: '\x22\x22' };
    oraMdField['F1_FILTERS_LBL']={ fieldValue: 'Filtros' };
    oraMdField['F1_EXPLORER_LBL']={ fieldValue: 'Explorador' };
    oraMdField['DE_GOTOZONE_LBL']={ fieldValue: 'Ir\x20Para\x20Zona' };

 
  
           
          
      
     
  
 

  

   document.addEventListener(&quot;DOMContentLoaded&quot;, function() {
     checkMultiQueryFilterVisibility(id('filterButton_1', document), 1);
   });

   if (!this.handleDashboardLoad) { 
     var elem = id('zoneMenu_1', document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
   } 
 /* if (trimString(id('helpText_1', document).innerHTML) != '') {
    oraInsertHelpForZoneHeaders(1);    
  }*/
 
   checkMultiQueryFilterVisibility(id('filterButton_1', document), 1); 
   checkDnDVisibility(id('dndButton_1', document), 1); 

   if (id('debug', main.document)) { 
     var elem = id('editZone_1', document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
   }
     
   if ('' != '0') { 
     var elem = id('exportZone_1', document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
   } 

var showSavePreferenes=false;
if((parent.main.userProfile.isTemplateUser == true) || (parent.main.userProfile.isTemplateUser == false &amp;&amp; parent.main.userProfile.portalProfileUserId == &quot;&quot;)){
showSavePreferenes= true;
}
 if(showSavePreferenes){
     var elem = id('savePreferences_1', document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
  }
  if(id('resetZonePreferences_1', document)){
     var elem = id('resetZonePreferences_1', document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
   }

function showMenu(evt, sequenceId) {
    if(window.currentSequence) 
        window.hideMenuImmediately(sequenceId);
    
    var menu = id('explorerZoneMenu_' + sequenceId, document);
    setStyle(menu, 'z-index', '20');
    
    if (id('dataExplorerDescription1', document)) { 
        setStyle(getFirstNonTextChild(menu).rows[3], 'display', ''); 
    } 
    if (id('debug', main.document)) { 
        setStyle(getFirstNonTextChild(menu).rows[3], 'display', ''); 
        setStyle(getFirstNonTextChild(menu).rows[4], 'display', '');
        setStyle(getFirstNonTextChild(menu).rows[5], 'display', '');
    } 
    setStyle(menu, 'top', (main.computeAbsoluteOffsetTop(getTarget(evt)) + 5) + 'px');
    setStyle(menu, 'left', (main.computeAbsoluteOffsetLeft(getTarget(evt)) + 5) + 'px');
    ouaf.core.CSSHelper.removeClassByElement(menu,'hide');
    window.currentSequence = sequenceId;
    function leave(e)
    {
        hideMenuImmediately(sequenceId);
    }
    
    if(!_IE) ffEmulator.addEvent(menu, 'mouseleave', leave, false);
    else addListener(id('explorerZoneMenu_' + sequenceId, document), 'mouseleave', leave );
    
    if(_IE) showFrame(menu, sequenceId);
}

function checkMultiQueryFilterVisibility(element, sequenceId) { 
    var location = &quot;&quot;; 
    var filters = &quot;&quot;; 
    if ((location.toUpperCase() == 'T') 
       || (filters.toUpperCase().indexOf(&quot;FILTERAREA=N&quot;) > -1)  ) { 
        ouaf.core.CSSHelper.addClassByElement(element,'hide');
        return;
    } 
    if(window.filterMap1){
         var filter_string = window.filterMap1['F1'] + window.filterMap1['F2'] + window.filterMap1['F3'] + window.filterMap1['F4'] + window.filterMap1['F5'] + window.filterMap1['F6'] + window.filterMap1['F7'] + window.filterMap1['F8'] + window.filterMap1['F9'] + window.filterMap1['F10'] + window.filterMap1['F11'] + window.filterMap1['F12'] + window.filterMap1['F13'] + window.filterMap1['F14'] + window.filterMap1['F15'];

         if(filter_string == &quot;&quot;)
           ouaf.core.CSSHelper.addClassByElement(element,'hide');
         else
           ouaf.core.CSSHelper.removeClassByElement(element,'hide');
    }
    else
           ouaf.core.CSSHelper.addClassByElement(element,'hide');
}

function checkDnDVisibility(element, sequenceId) { 
    var dnd = &quot;&quot;; 
    if (   (this.handleDashboardLoad)  
       || (dnd.toUpperCase().indexOf(&quot;DRAGDROPAREA=N&quot;) > -1)  
       ) { 
        return; 
    } 
    ouaf.core.CSSHelper.removeClassByElement(element,'hide');
} 

// other related functions have been moved to userMapSupport.js


 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField['F1_EXPLORER_ZONE_MENU_LBL']={ fieldValue: 'Menu\x20da\x20Zona\x20do\x20Explorador' };
    oraMdField['F1_OPEN_SAVED_SEARCH_LBL']={ fieldValue: 'Open\x20Saved\x20Search\x2E\x2E\x2E' };
    oraMdField['F1_CLEAR_PREFERENCES']={ fieldValue: 'Clear\x20Filters' };
    oraMdField['DE_EXP_TO_EXCEL_LBL']={ fieldValue: 'Exportar\x20Para\x20Excel' };
    oraMdField['DE_PRINT_LBL']={ fieldValue: 'Imprimir\x20Zona' };
    oraMdField['DE_SHOW_XML_LBL']={ fieldValue: 'Mostrar\x20Dados\x20do\x20Serviço' };
    oraMdField['DE_SHOW_SQL_LBL']={ fieldValue: 'Mostrar\x20SQL' };
    oraMdField['F1_SAVE']={ fieldValue: 'Save' };
    oraMdField['F1_SAVE_AS']={ fieldValue: 'Save\x20As\x20\x2E\x2E\x2E' };
    oraMdField['F1_DELETE']={ fieldValue: 'Delete' };
    oraMdField['F1_SET_AS_DEFAULT']={ fieldValue: 'Set\x20As\x20Default' };
    oraMdField['F1_REMOVE_AS_DEFAULT']={ fieldValue: 'Remove\x20As\x20Default' };
    oraMdField['F1_ADD_TO_FAVORITES']={ fieldValue: 'Add\x20To\x20Favorites' };
    oraMdField['F1_REMOVE_FROM_FAVORITES']={ fieldValue: 'Remove\x20From\x20Favorites' };
    oraMdField['DE_PRINT_LBL_STANDARD']={ fieldValue: 'Standard' };
    oraMdField['DE_PRINT_LBL_FORMATTED']={ fieldValue: 'Formatted' };

 main.zoneMenuWindow = getDefaultView(document); 

    

   
 
	 
	 
               Open Saved Search...
			   
					
			   
			   
     
     
               Clear Filters
			   
			   
     
  
	  
			 Exportar Para Excel

			   			 
	  
		 
      
         Imprimir Zona
		 
                       
			   
      
	 
      
         Mostrar Dados do Serviço
		 
	         
      
	 
      
         Mostrar SQL 
		 
			   
      
	  
	  
         Save
		 
			   
      
	  
         Save As ...
		 
			   
      
	  
         Delete 
		 
			   
      
	  
         Set As Default
		 
			   
      
	  
         Remove As Default
		 
			   
      
	  
         Add To Favorites
		 
			   
      
      
	  
         Remove From Favorites
		 
			   
      







 
      
         Standard
		 
			   
      

      
         Formatted
		 
			   
      






var loadedSavedSearchName_1;


function lazyLoad_1(seqNumber, rowsToExport) {
    if (!this.handleDashboardLoad) {
        var elem = id('zoneMenu_1', document);
        ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
    } 
    if (trimString(id('helpText_1', document).innerHTML) != '') {
        main.zoneMenuWindow.oraInsertHelpForZoneHeaders(1);    
    }
    else {
        main.zoneMenuWindow.oraInsertSavedSearchTitle(null,1);  
    }
    updateZoneMenuHTML(seqNumber, rowsToExport);
}

setTimeout(lazyLoad_1, 200);
setTimeout(lazyLoad_1(1,''), 200);

 
function buildNameSearch_1(document,sequenceId){
var savedSearchNames =getDefaultView(document)['savedSearches'+sequenceId];
var innerListItems =&quot;&quot;;
  if(savedSearchNames){
             if (savedSearchNames.length > 0) {

   	       for(var i=0;i&lt;savedSearchNames.length;i++){
		 var searchName=savedSearchNames[i].name;
		 if(searchName){
  		   var lineItem=&quot;&lt;li id=\&quot;savedItem_&quot;+i+&quot;\&quot; tabindex=\&quot;5\&quot; data-menuType=\&quot;HTML\&quot; class=\&quot;menuItem\&quot; role=\&quot;menuitem\&quot; tabIndex=\&quot;5\&quot; onclick=\&quot;ouaf.context.expandSubMenu(true); main.zoneMenuWindow.loadNameSearch(main.zoneMenuWindow.document,1); main.zoneMenuWindow.hideMenuImmediately(1); return false;\&quot;  onkeypress=\&quot; ouaf.context.onMenuKeyPress(event); \&quot; onkeydown=\&quot;ouaf.context.onMenuKeyPress(event)\&quot; onmouseover=\&quot;ouaf.context.expandSubMenu(false)\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot; data-isLeaf=\&quot;true\&quot;>&quot;;
                   if (savedSearchNames[i].isDefault) {
						lineItem = lineItem+&quot;&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;img tabindex=\&quot;5\&quot;  id=\&quot;defaultSearch_1\&quot; src=\&quot;images/star.gif\&quot; />&quot;+searchName+&quot;&lt;/span>&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;/li>&quot;;
                   }else{
						lineItem = lineItem+&quot;&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&quot;+searchName+&quot;&lt;/span>&lt;span  data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;/li>&quot;;
				   }
                   
   	           innerListItems = innerListItems + lineItem;
	    	 }
               }
             } else {
                var elem = id('savedSearch_1',document);
                ouaf.core.CSSHelper.addClassByElement(elem,'hide');
             }
  } else {
                var elem = id('savedSearch_1',document);
                ouaf.core.CSSHelper.addClassByElement(elem,'hide');
  }
  if(document.getElementById('savedSearchSubMenuList_'+sequenceId)){
    document.getElementById('savedSearchSubMenuList_'+sequenceId).innerHTML = innerListItems;
  }
} 


function updateZoneMenuHTML(seqNum, rowsToExport){
   if (rowsToExport != '0') { 
     var elem = id('exportZone_'+ seqNum, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,'hide');
   } 
}   

function showMenuFusion_1(evt, sequenceId) {
  main.doClose();
  main.zoneMenuWindow = getDefaultView(document);
  main.applyMenuItemsDisplaySettings(document, sequenceId);
  var explZoneMenu = main.ouaf.ui.Menu({&quot;menuType&quot;:&quot;HTML&quot;, &quot;menuName&quot;:&quot;CI_ZONEMENU&quot;, &quot;menuModel&quot;:{&quot;html&quot;: document.getElementById(&quot;explorerZoneMenu_1&quot;).innerHTML}, &quot;savedSearchModel_1&quot;: {&quot;html&quot;: document.getElementById('savedSearchesSubmenu_1').innerHTML}, &quot;printMenuModel_1&quot;: {&quot;html&quot;: document.getElementById('printSubmenu_1').innerHTML}});
  main.menuObjectsMap[explZoneMenu.name]= explZoneMenu;
  
  explZoneMenu.toggle(evt);
  
}

  function clearLoadedSavedSearchName_1(sequenceId) {
    main.top.tabPage()['loadedSavedSearchName_'+sequenceId] = '';
  }

  document.addEventListener(&quot;DOMContentLoaded&quot;, function(event) {
        var buildMenuFunction = getDefaultView(document)['buildNameSearch_1'];
        if (buildMenuFunction) {
            buildMenuFunction(document, 1);
        }
  });
Acrescentar


 
 var userZonePreferences1={};
var savedSearches1=[];




      var sqlData = { 
    A: {C1: { type:  
    'D'
   
            , width: 0
            }
       }
   }; 
    














Número da Mensagem
                     ,










Não existem informações para apresentar





Pesquisa por
      
         Pessoa
	Data Receção / Tipo Formul. / Estado / Tipo Declaração
	Endereço
	ID Batch Formulários Externo, Cabeçalho Batch Formulários
	Localização do Documento
	







    var oraSchemaInfo2={name:'CM\x2DFormQueryFilter8',type:'F1MP',children:[{name:'entityNameValue',mdField:'ENTITY\x5FNAME',label:'Nome',dataType:'string',precision:254},{name:'fullNameValue',mdField:'FULL\x5FNAME',label:'Nome\x20Completo',dataType:'string',precision:254},{name:'firstNameValue',mdField:'C1\x5FFIRST\x5FNAME',label:'Nome\x20Próprio',dataType:'string',precision:50},{name:'lastNameValue',mdField:'C1\x5FLAST\x5FNAME',label:'Apelido',dataType:'string',precision:50},{name:'idType',mdField:'ID\x5FTYPE\x5FCD',label:'Tipo\x20de\x20Identificação',dataType:'string',precision:8},{name:'idValue',mdField:'PER\x5FID\x5FVAL',label:'Valor\x20da\x20ID',dataType:'string',precision:16},{name:'personType',label:'\x28personType\x29',dataType:'string'},{name:'isExtendedNameDetailsSupported',label:'\x28isExtendedNameDetailsSupported\x29',dataType:'boolean'},{name:'name',mdField:'ENTITY\x5FNAME',label:'Nome',dataType:'string',precision:254}]}


function table_CIPCIDFW() {
	return [
		{key: &quot;AID&quot;, value: &quot;999999999XX999\x2C\x20where\x209\x27s\x20repr&quot;}
	];
} 
 document.table_CIPCIDFW = table_CIPCIDFW();

 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField['ENTITY_NAME']={ fieldValue: 'Nome' };
    oraMdField['FULL_NAME']={ fieldValue: 'Nome\x20Completo' };
    oraMdField['FIRST_NAME']={ fieldValue: 'Primeiro\x20Nome' };
    oraMdField['LAST_NAME']={ fieldValue: 'Último\x20Nome' };
    oraMdField['ID_TYPE_LBL']={ fieldValue: 'Tipo\x20de\x20ID' };
    oraMdField['PER_ID_VAL']={ fieldValue: 'Valor\x20da\x20ID' };



var FORMAT_ID_JS_FILENAME = &quot;code/formatValue.js&quot;;
var IS_EXT_NAME_SUPPORTED = &quot;false&quot;;

function oraLoad1() {

/* load FW formatValue.js dynamically since using script tag
* and pointing it to a JS source is not allowed in fragment
* UI Maps
*/
var fileref = document.createElement('script');
fileref.setAttribute(&quot;type&quot;, &quot;text/javascript&quot;);
fileref.setAttribute(&quot;src&quot;, FORMAT_ID_JS_FILENAME);
id('filterUIFields').appendChild(fileref);
invokeSS();

}


function formatIdNumber() {
var personIdNbr = id('personIdValue').value;
var idTypeCode = id('personIdType').value;
var idFormat = &quot;&quot;;
if (personIdNbr == &quot;&quot; || idTypeCode == &quot;&quot;) {
return;
}
for (var i = 0; i &lt; document.table_CIPCIDFW.length; i++) {

if (document.table_CIPCIDFW[i].key == idTypeCode) {
idFormat = document.table_CIPCIDFW[i].value;
break;
}
}
id('personIdValue').value = formatValue(personIdNbr, idFormat);
}

function updateName() {
if (IS_EXT_NAME_SUPPORTED == 'true')
id('nameValue').value = id('fullNameValue').value;
else
id('nameValue').value = id('entityNameValue').value;
}


function invokeSS() {

var error = '';
try {
var doc = createXMLDoc(null, null, selectionNamespaces);
var extendedPersonNameDetails = doc.createElement('extendedPersonNameDetails');
doc.appendChild(extendedPersonNameDetails);
extendedPersonNameDetails.setAttribute('type', 'group');

var personType = doc.createElement('personType');
extendedPersonNameDetails.appendChild(personType);
personType.appendChild(doc.createTextNode(''));

var isExtendedNameDetailsSupported = doc.createElement('isExtendedNameDetailsSupported');
extendedPersonNameDetails.appendChild(isExtendedNameDetailsSupported);
extendedPersonNameDetails.appendChild(doc.createTextNode(''));

var ssResults = main[&quot;invokeSS&quot;](&quot;C1-ChkExNmDt&quot;, doc, null, true, null);

var results = ssResults.getElementsByTagName('isExtendedNameDetailsSupported');

if (results[0].childNodes[0].nodeValue == 'true') {
IS_EXT_NAME_SUPPORTED = 'true';
id('tableRow1').style.display = 'none';
} else {

id('tableRow2').style.display = 'none';
id('tableRow3').style.display = 'none';
id('tableRow4').style.display = 'none';

}

} catch (err) {
alert('DEBUG: ' + err.description + error); //for debugging..
}
}

var oldOnload = window.onload;

function init() {
oldOnload();
}

window.onload = init;






   function getEntityNameFieldSize(){
       //Defines the standard Entity Name element size across UI-Maps
       return 80;
   }


   function getEntityNameMaxlength(){
        //Defines the standard Entity Name element size across UI-Maps
        return 254
   }

   function setTextFieldMaxlength(inputTextField){
    if(!inputTextField.getAttribute('maxlength')){
        addHTMLAttribute(inputTextField, 'maxlength',  getEntityNameMaxlength());
    }
   }

    function addHTMLAttribute( htmlElement, attribName, attribValue ){
        if(htmlElement){
            htmlElement.setAttribute(attribName, attribValue);
        }
    }

    function expandTextFieldWidth( elementId ){        
    
        var inputTextField = id(elementId)

        if(inputTextField){              
                addHTMLAttribute(inputTextField, 'size',  getEntityNameFieldSize());
        }

        setTextFieldMaxlength(inputTextField);

    }



Nome





Nome Completo





Primeiro Nome



Último Nome



Tipo de ID


Bilhete de IdentidadeAssento de NascimentoCertidão de NascimentoNº de Registo ComercialCarta de ConduçãoIdentificação de Cidadão EstrangeiroBI LegadoNIFOutroPassaporteIdentificação de Funcionário PúblicoCartão de RefugiadoCartão de ResidenteCartão de Eleitor
 Valor da ID 






999999999XX999, where 9's repr








          window.filterMap1 = {};
          window.filterMap1['F1'] = &quot;type=string xpath=name likeable=PS label=FULL_NAME&quot;;
          window.filterMap1['F2'] = &quot;type=string xpath=firstNameValue likeable=S label=FIRST_NAME&quot;;
          window.filterMap1['F3'] = &quot;type=string xpath=lastNameValue likeable=S label=C1_LAST_NAME&quot;;
          window.filterMap1['F4'] = &quot;type=string xpath=idType label=ID_TYPE_CD likeable=S&quot;;
          window.filterMap1['F5'] = &quot;type=string xpath=idValue label=PER_ID_VAL&quot;;
          window.filterMap1['F6'] = &quot;&quot;;
          window.filterMap1['F7'] = &quot;&quot;;
          window.filterMap1['F8'] = &quot;&quot;;
          window.filterMap1['F9'] = &quot;&quot;;
          window.filterMap1['F10'] = &quot;&quot;;
          window.filterMap1['F11'] = &quot;&quot;;
          window.filterMap1['F12'] = &quot;&quot;;
          window.filterMap1['F13'] = &quot;&quot;;
          window.filterMap1['F14'] = &quot;&quot;;
          window.filterMap1['F15'] = &quot;&quot;;
          window.filterMap1['F16'] = &quot;&quot;;
          window.filterMap1['F17'] = &quot;&quot;;
          window.filterMap1['F18'] = &quot;&quot;;
          window.filterMap1['F19'] = &quot;&quot;;
          window.filterMap1['F20'] = &quot;&quot;;
          window.filterMap1['F21'] = &quot;&quot;;
          window.filterMap1['F22'] = &quot;&quot;;
          window.filterMap1['F23'] = &quot;&quot;;
          window.filterMap1['F24'] = &quot;&quot;;
          window.filterMap1['F25'] = &quot;&quot;;
          
      function refreshZone1(isFromAutoRefresh) {
        if(isFromAutoRefresh){
            initializeDEZone('C1-TXFRQRY', 1, isFromAutoRefresh, null, window.filterMap1);
        }else{
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, false);
      }
          
      } 
      function refreshZone1ForBroadcast() {
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, true);
      }
      function storeZoneParameters1(document, _portalState, isFromSavePref) {
          parent.storeDataExplorerZoneParameters(1, document, _portalState, isFromSavePref);
      }
      
       function refreshZone1ForPagination(pageDirection) {
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, false,pageDirection);
      }
     
     function getSearchButtonElement1(document){
      	   var inputSeq = 1;
           var idForSearchButton = &quot;anTLZ&quot; + inputSeq + &quot;Refresh&quot;;
		   var searchButton = id(idForSearchButton, document);
		   if (main.accessibilityFeatureEnabled &amp;&amp; !searchButton) {
		    	idForSearchButton = &quot;anTLZ&quot; + inputSeq + &quot;Refresh_dnd&quot;;
		    	searchButton = id(idForSearchButton, document);
		    	
		  }
		  return searchButton;
	}
     
      function positionSearchButton1() {
		    if (!isReadyStateComplete(window)) {
		        window.setTimeout(positionSearchButton1, 200);
		        return;
		    }
		    var inputSeq = 1;
		    var inputElem = getSearchButtonElement1(document);  
		    if (!inputElem) {
		        return;
		    }	
		    
		    var filterMSeq = 1;
		    var spanFilterId = &quot;zoneBody&quot; + filterMSeq;
		    var dataExplorerFiltersId = &quot;dataExplorerFilters&quot; + filterMSeq;
		    var multiQueryFilterTableId = &quot;multiQueryFilterBody&quot; + inputSeq; //multiQueryFilterBody{$sequenceId}
		
		    //pick last element from the filterUiMap
		    var lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + spanFilterId + &quot; :input[orafield]&quot;, document);
		    var lstLabel = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + spanFilterId + &quot; label[oraLabel], label[oraMdLabel]&quot;, document);
		
		    //now search the default filter elements from multiQueryFilterTable
		    if (lstElems.length == 0) {
		        lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + multiQueryFilterTableId + &quot; :input[id^=filter]&quot;, document); //elements whose id starting with filter
		    }
		    
		    if(lstElems.length == 0){
		    	lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + dataExplorerFiltersId + &quot; :input[id^=filter]&quot;, document);
		    }
		    
		    var isLTRdirection = true;
		     
		    var pos, i = 0, j = 0;
		    for (i = 0; i &lt; lstElems.length; i++) {
		    	if(!ouaf.core.CSSHelper.isElementHidden(lstElems[i])){
		    		if(isLTRdirection)
			        	pos =  lstElems[i].parentNode.offsetLeft;											
			        else
			        	pos = ouaf.dom.DOMHelper.getFarEnd(lstElems[i]);
			        if (pos > 0) {
			            break;
			        }
		        }
		    }
		    if(!pos){
			    for (j = 0; j &lt; lstLabel.length; j++) {
		    	    if(!ouaf.core.CSSHelper.isElementHidden(lstLabel[j])){
		    		    if(isLTRdirection)
							pos = ouaf.dom.DOMHelper.getFarEnd(lstLabel[j]); 												
						if (pos > 0) {
							break;
						}	                   
					}
				}
			}	
		    
		    ouaf.core.CSSHelper.removeClassByElement(inputElem, &quot;hide&quot;);//display the Search button before returning
		    if (i > lstElems.length || lstElems.length==0) return; //found no element in the list
		    
			if(!isLTRdirection &amp;&amp; ouaf.dom.DOMHelper.getFarEnd(inputElem) > pos){
				pos=ouaf.dom.DOMHelper.getFarEnd(inputElem)-pos;
				ouaf.core.CSSHelper.setStyleByElement(inputElem, {&quot;margin-right&quot;:  pos+&quot;px&quot;});	
			}else{	       
			    ouaf.core.CSSHelper.setStyleByElement(inputElem, {&quot;margin-left&quot;: pos + &quot;px&quot;});
		    }
		
		}
		window.setTimeout(positionSearchButton1, 200);
      
  




















































































          		var func1 = function(){
                	parent.handleDataExplorerZoneLoad(document, 1, true);
          		};
          		func1();
          








/html[1]</value>
      <webElementGuid>1c616709-9a5b-4a98-b8a8-d589059128f3</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
      <webElementGuid>e511828a-5be4-4129-a938-e4c4fe314d32</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/SIGT/Page_Consulta de Formulrios Fiscais/iframe_Consulta de Formulrios Fiscais_tabPage</value>
      <webElementGuid>8d38f224-acaf-4d69-aa66-14f2c939155f</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
      <webElementGuid>01b6195d-c657-456b-b126-71e675f28f56</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
      <webElementGuid>480f0263-9064-4c17-9baa-eebd9217d7ef</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//html[(text() = concat(&quot;




var main = parent;
var myPos = parent;
var noOfZones = 1;
 var refreshPeriodList=[]; var zoneIntervals=[]; refreshPeriodList[1]=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;var portalOptions={&quot;lists&quot;:{&quot;optionValues&quot;:{&quot;name&quot;:&quot;optionValues&quot;,&quot;header&quot;:{&quot;moreRows&quot;:false,&quot;remainingRows&quot;:0,&quot;lastInfo&quot;:&quot;&quot;},&quot;list&quot;:[]}}};var portalMOAndKeys = {};pushMOAndKeysIfAvailable();
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

main.pendingGlobalContext.waitForZone(0);



Pesquisa de Formulários Fiscais

 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField[&quot; , &quot;'&quot; , &quot;EMPTY_SUMMARY&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;\x22\x22&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_FILTERS_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Filtros&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_EXPLORER_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Explorador&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_GOTOZONE_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Ir\x20Para\x20Zona&quot; , &quot;'&quot; , &quot; };

 
  
           
          
      
     
  
 

  

   document.addEventListener(&quot;DOMContentLoaded&quot;, function() {
     checkMultiQueryFilterVisibility(id(&quot; , &quot;'&quot; , &quot;filterButton_1&quot; , &quot;'&quot; , &quot;, document), 1);
   });

   if (!this.handleDashboardLoad) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;zoneMenu_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   } 
 /* if (trimString(id(&quot; , &quot;'&quot; , &quot;helpText_1&quot; , &quot;'&quot; , &quot;, document).innerHTML) != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
    oraInsertHelpForZoneHeaders(1);    
  }*/
 
   checkMultiQueryFilterVisibility(id(&quot; , &quot;'&quot; , &quot;filterButton_1&quot; , &quot;'&quot; , &quot;, document), 1); 
   checkDnDVisibility(id(&quot; , &quot;'&quot; , &quot;dndButton_1&quot; , &quot;'&quot; , &quot;, document), 1); 

   if (id(&quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;, main.document)) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;editZone_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   }
     
   if (&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; != &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;exportZone_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   } 

var showSavePreferenes=false;
if((parent.main.userProfile.isTemplateUser == true) || (parent.main.userProfile.isTemplateUser == false &amp;&amp; parent.main.userProfile.portalProfileUserId == &quot;&quot;)){
showSavePreferenes= true;
}
 if(showSavePreferenes){
     var elem = id(&quot; , &quot;'&quot; , &quot;savePreferences_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
  }
  if(id(&quot; , &quot;'&quot; , &quot;resetZonePreferences_1&quot; , &quot;'&quot; , &quot;, document)){
     var elem = id(&quot; , &quot;'&quot; , &quot;resetZonePreferences_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   }

function showMenu(evt, sequenceId) {
    if(window.currentSequence) 
        window.hideMenuImmediately(sequenceId);
    
    var menu = id(&quot; , &quot;'&quot; , &quot;explorerZoneMenu_&quot; , &quot;'&quot; , &quot; + sequenceId, document);
    setStyle(menu, &quot; , &quot;'&quot; , &quot;z-index&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;);
    
    if (id(&quot; , &quot;'&quot; , &quot;dataExplorerDescription1&quot; , &quot;'&quot; , &quot;, document)) { 
        setStyle(getFirstNonTextChild(menu).rows[3], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); 
    } 
    if (id(&quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;, main.document)) { 
        setStyle(getFirstNonTextChild(menu).rows[3], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); 
        setStyle(getFirstNonTextChild(menu).rows[4], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        setStyle(getFirstNonTextChild(menu).rows[5], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    } 
    setStyle(menu, &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, (main.computeAbsoluteOffsetTop(getTarget(evt)) + 5) + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
    setStyle(menu, &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, (main.computeAbsoluteOffsetLeft(getTarget(evt)) + 5) + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
    ouaf.core.CSSHelper.removeClassByElement(menu,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
    window.currentSequence = sequenceId;
    function leave(e)
    {
        hideMenuImmediately(sequenceId);
    }
    
    if(!_IE) ffEmulator.addEvent(menu, &quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, leave, false);
    else addListener(id(&quot; , &quot;'&quot; , &quot;explorerZoneMenu_&quot; , &quot;'&quot; , &quot; + sequenceId, document), &quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, leave );
    
    if(_IE) showFrame(menu, sequenceId);
}

function checkMultiQueryFilterVisibility(element, sequenceId) { 
    var location = &quot;&quot;; 
    var filters = &quot;&quot;; 
    if ((location.toUpperCase() == &quot; , &quot;'&quot; , &quot;T&quot; , &quot;'&quot; , &quot;) 
       || (filters.toUpperCase().indexOf(&quot;FILTERAREA=N&quot;) > -1)  ) { 
        ouaf.core.CSSHelper.addClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
        return;
    } 
    if(window.filterMap1){
         var filter_string = window.filterMap1[&quot; , &quot;'&quot; , &quot;F1&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F2&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F3&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F4&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F5&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F6&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F7&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F8&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F9&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F10&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F11&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F12&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F13&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F14&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F15&quot; , &quot;'&quot; , &quot;];

         if(filter_string == &quot;&quot;)
           ouaf.core.CSSHelper.addClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
         else
           ouaf.core.CSSHelper.removeClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
    }
    else
           ouaf.core.CSSHelper.addClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
}

function checkDnDVisibility(element, sequenceId) { 
    var dnd = &quot;&quot;; 
    if (   (this.handleDashboardLoad)  
       || (dnd.toUpperCase().indexOf(&quot;DRAGDROPAREA=N&quot;) > -1)  
       ) { 
        return; 
    } 
    ouaf.core.CSSHelper.removeClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
} 

// other related functions have been moved to userMapSupport.js


 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_EXPLORER_ZONE_MENU_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Menu\x20da\x20Zona\x20do\x20Explorador&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_OPEN_SAVED_SEARCH_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Open\x20Saved\x20Search\x2E\x2E\x2E&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_CLEAR_PREFERENCES&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Clear\x20Filters&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_EXP_TO_EXCEL_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Exportar\x20Para\x20Excel&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_PRINT_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Imprimir\x20Zona&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_SHOW_XML_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Mostrar\x20Dados\x20do\x20Serviço&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_SHOW_SQL_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Mostrar\x20SQL&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_SAVE&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Save&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_SAVE_AS&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Save\x20As\x20\x2E\x2E\x2E&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_DELETE&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Delete&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_SET_AS_DEFAULT&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Set\x20As\x20Default&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_REMOVE_AS_DEFAULT&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Remove\x20As\x20Default&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_ADD_TO_FAVORITES&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Add\x20To\x20Favorites&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_REMOVE_FROM_FAVORITES&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Remove\x20From\x20Favorites&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_PRINT_LBL_STANDARD&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Standard&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_PRINT_LBL_FORMATTED&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Formatted&quot; , &quot;'&quot; , &quot; };

 main.zoneMenuWindow = getDefaultView(document); 

    

   
 
	 
	 
               Open Saved Search...
			   
					
			   
			   
     
     
               Clear Filters
			   
			   
     
  
	  
			 Exportar Para Excel

			   			 
	  
		 
      
         Imprimir Zona
		 
                       
			   
      
	 
      
         Mostrar Dados do Serviço
		 
	         
      
	 
      
         Mostrar SQL 
		 
			   
      
	  
	  
         Save
		 
			   
      
	  
         Save As ...
		 
			   
      
	  
         Delete 
		 
			   
      
	  
         Set As Default
		 
			   
      
	  
         Remove As Default
		 
			   
      
	  
         Add To Favorites
		 
			   
      
      
	  
         Remove From Favorites
		 
			   
      







 
      
         Standard
		 
			   
      

      
         Formatted
		 
			   
      






var loadedSavedSearchName_1;


function lazyLoad_1(seqNumber, rowsToExport) {
    if (!this.handleDashboardLoad) {
        var elem = id(&quot; , &quot;'&quot; , &quot;zoneMenu_1&quot; , &quot;'&quot; , &quot;, document);
        ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
    } 
    if (trimString(id(&quot; , &quot;'&quot; , &quot;helpText_1&quot; , &quot;'&quot; , &quot;, document).innerHTML) != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
        main.zoneMenuWindow.oraInsertHelpForZoneHeaders(1);    
    }
    else {
        main.zoneMenuWindow.oraInsertSavedSearchTitle(null,1);  
    }
    updateZoneMenuHTML(seqNumber, rowsToExport);
}

setTimeout(lazyLoad_1, 200);
setTimeout(lazyLoad_1(1,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;), 200);

 
function buildNameSearch_1(document,sequenceId){
var savedSearchNames =getDefaultView(document)[&quot; , &quot;'&quot; , &quot;savedSearches&quot; , &quot;'&quot; , &quot;+sequenceId];
var innerListItems =&quot;&quot;;
  if(savedSearchNames){
             if (savedSearchNames.length > 0) {

   	       for(var i=0;i&lt;savedSearchNames.length;i++){
		 var searchName=savedSearchNames[i].name;
		 if(searchName){
  		   var lineItem=&quot;&lt;li id=\&quot;savedItem_&quot;+i+&quot;\&quot; tabindex=\&quot;5\&quot; data-menuType=\&quot;HTML\&quot; class=\&quot;menuItem\&quot; role=\&quot;menuitem\&quot; tabIndex=\&quot;5\&quot; onclick=\&quot;ouaf.context.expandSubMenu(true); main.zoneMenuWindow.loadNameSearch(main.zoneMenuWindow.document,1); main.zoneMenuWindow.hideMenuImmediately(1); return false;\&quot;  onkeypress=\&quot; ouaf.context.onMenuKeyPress(event); \&quot; onkeydown=\&quot;ouaf.context.onMenuKeyPress(event)\&quot; onmouseover=\&quot;ouaf.context.expandSubMenu(false)\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot; data-isLeaf=\&quot;true\&quot;>&quot;;
                   if (savedSearchNames[i].isDefault) {
						lineItem = lineItem+&quot;&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;img tabindex=\&quot;5\&quot;  id=\&quot;defaultSearch_1\&quot; src=\&quot;images/star.gif\&quot; />&quot;+searchName+&quot;&lt;/span>&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;/li>&quot;;
                   }else{
						lineItem = lineItem+&quot;&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&quot;+searchName+&quot;&lt;/span>&lt;span  data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;/li>&quot;;
				   }
                   
   	           innerListItems = innerListItems + lineItem;
	    	 }
               }
             } else {
                var elem = id(&quot; , &quot;'&quot; , &quot;savedSearch_1&quot; , &quot;'&quot; , &quot;,document);
                ouaf.core.CSSHelper.addClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
             }
  } else {
                var elem = id(&quot; , &quot;'&quot; , &quot;savedSearch_1&quot; , &quot;'&quot; , &quot;,document);
                ouaf.core.CSSHelper.addClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
  }
  if(document.getElementById(&quot; , &quot;'&quot; , &quot;savedSearchSubMenuList_&quot; , &quot;'&quot; , &quot;+sequenceId)){
    document.getElementById(&quot; , &quot;'&quot; , &quot;savedSearchSubMenuList_&quot; , &quot;'&quot; , &quot;+sequenceId).innerHTML = innerListItems;
  }
} 


function updateZoneMenuHTML(seqNum, rowsToExport){
   if (rowsToExport != &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;exportZone_&quot; , &quot;'&quot; , &quot;+ seqNum, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   } 
}   

function showMenuFusion_1(evt, sequenceId) {
  main.doClose();
  main.zoneMenuWindow = getDefaultView(document);
  main.applyMenuItemsDisplaySettings(document, sequenceId);
  var explZoneMenu = main.ouaf.ui.Menu({&quot;menuType&quot;:&quot;HTML&quot;, &quot;menuName&quot;:&quot;CI_ZONEMENU&quot;, &quot;menuModel&quot;:{&quot;html&quot;: document.getElementById(&quot;explorerZoneMenu_1&quot;).innerHTML}, &quot;savedSearchModel_1&quot;: {&quot;html&quot;: document.getElementById(&quot; , &quot;'&quot; , &quot;savedSearchesSubmenu_1&quot; , &quot;'&quot; , &quot;).innerHTML}, &quot;printMenuModel_1&quot;: {&quot;html&quot;: document.getElementById(&quot; , &quot;'&quot; , &quot;printSubmenu_1&quot; , &quot;'&quot; , &quot;).innerHTML}});
  main.menuObjectsMap[explZoneMenu.name]= explZoneMenu;
  
  explZoneMenu.toggle(evt);
  
}

  function clearLoadedSavedSearchName_1(sequenceId) {
    main.top.tabPage()[&quot; , &quot;'&quot; , &quot;loadedSavedSearchName_&quot; , &quot;'&quot; , &quot;+sequenceId] = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
  }

  document.addEventListener(&quot;DOMContentLoaded&quot;, function(event) {
        var buildMenuFunction = getDefaultView(document)[&quot; , &quot;'&quot; , &quot;buildNameSearch_1&quot; , &quot;'&quot; , &quot;];
        if (buildMenuFunction) {
            buildMenuFunction(document, 1);
        }
  });
Acrescentar


 
 var userZonePreferences1={};
var savedSearches1=[];




      var sqlData = { 
    A: {C1: { type:  
    &quot; , &quot;'&quot; , &quot;D&quot; , &quot;'&quot; , &quot;
   
            , width: 0
            }
       }
   }; 
    














Número da Mensagem
                     ,










Não existem informações para apresentar





Pesquisa por
      
         Pessoa
	Data Receção / Tipo Formul. / Estado / Tipo Declaração
	Endereço
	ID Batch Formulários Externo, Cabeçalho Batch Formulários
	Localização do Documento
	







    var oraSchemaInfo2={name:&quot; , &quot;'&quot; , &quot;CM\x2DFormQueryFilter8&quot; , &quot;'&quot; , &quot;,type:&quot; , &quot;'&quot; , &quot;F1MP&quot; , &quot;'&quot; , &quot;,children:[{name:&quot; , &quot;'&quot; , &quot;entityNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;ENTITY\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:254},{name:&quot; , &quot;'&quot; , &quot;fullNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;FULL\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome\x20Completo&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:254},{name:&quot; , &quot;'&quot; , &quot;firstNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;C1\x5FFIRST\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome\x20Próprio&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:50},{name:&quot; , &quot;'&quot; , &quot;lastNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;C1\x5FLAST\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Apelido&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:50},{name:&quot; , &quot;'&quot; , &quot;idType&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;ID\x5FTYPE\x5FCD&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Tipo\x20de\x20Identificação&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:8},{name:&quot; , &quot;'&quot; , &quot;idValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;PER\x5FID\x5FVAL&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Valor\x20da\x20ID&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:16},{name:&quot; , &quot;'&quot; , &quot;personType&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;\x28personType\x29&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;},{name:&quot; , &quot;'&quot; , &quot;isExtendedNameDetailsSupported&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;\x28isExtendedNameDetailsSupported\x29&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;boolean&quot; , &quot;'&quot; , &quot;},{name:&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;ENTITY\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:254}]}


function table_CIPCIDFW() {
	return [
		{key: &quot;AID&quot;, value: &quot;999999999XX999\x2C\x20where\x209\x27s\x20repr&quot;}
	];
} 
 document.table_CIPCIDFW = table_CIPCIDFW();

 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField[&quot; , &quot;'&quot; , &quot;ENTITY_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Nome&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;FULL_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Nome\x20Completo&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;FIRST_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Primeiro\x20Nome&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;LAST_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Último\x20Nome&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;ID_TYPE_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Tipo\x20de\x20ID&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;PER_ID_VAL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Valor\x20da\x20ID&quot; , &quot;'&quot; , &quot; };



var FORMAT_ID_JS_FILENAME = &quot;code/formatValue.js&quot;;
var IS_EXT_NAME_SUPPORTED = &quot;false&quot;;

function oraLoad1() {

/* load FW formatValue.js dynamically since using script tag
* and pointing it to a JS source is not allowed in fragment
* UI Maps
*/
var fileref = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);
fileref.setAttribute(&quot;type&quot;, &quot;text/javascript&quot;);
fileref.setAttribute(&quot;src&quot;, FORMAT_ID_JS_FILENAME);
id(&quot; , &quot;'&quot; , &quot;filterUIFields&quot; , &quot;'&quot; , &quot;).appendChild(fileref);
invokeSS();

}


function formatIdNumber() {
var personIdNbr = id(&quot; , &quot;'&quot; , &quot;personIdValue&quot; , &quot;'&quot; , &quot;).value;
var idTypeCode = id(&quot; , &quot;'&quot; , &quot;personIdType&quot; , &quot;'&quot; , &quot;).value;
var idFormat = &quot;&quot;;
if (personIdNbr == &quot;&quot; || idTypeCode == &quot;&quot;) {
return;
}
for (var i = 0; i &lt; document.table_CIPCIDFW.length; i++) {

if (document.table_CIPCIDFW[i].key == idTypeCode) {
idFormat = document.table_CIPCIDFW[i].value;
break;
}
}
id(&quot; , &quot;'&quot; , &quot;personIdValue&quot; , &quot;'&quot; , &quot;).value = formatValue(personIdNbr, idFormat);
}

function updateName() {
if (IS_EXT_NAME_SUPPORTED == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
id(&quot; , &quot;'&quot; , &quot;nameValue&quot; , &quot;'&quot; , &quot;).value = id(&quot; , &quot;'&quot; , &quot;fullNameValue&quot; , &quot;'&quot; , &quot;).value;
else
id(&quot; , &quot;'&quot; , &quot;nameValue&quot; , &quot;'&quot; , &quot;).value = id(&quot; , &quot;'&quot; , &quot;entityNameValue&quot; , &quot;'&quot; , &quot;).value;
}


function invokeSS() {

var error = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
try {
var doc = createXMLDoc(null, null, selectionNamespaces);
var extendedPersonNameDetails = doc.createElement(&quot; , &quot;'&quot; , &quot;extendedPersonNameDetails&quot; , &quot;'&quot; , &quot;);
doc.appendChild(extendedPersonNameDetails);
extendedPersonNameDetails.setAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;group&quot; , &quot;'&quot; , &quot;);

var personType = doc.createElement(&quot; , &quot;'&quot; , &quot;personType&quot; , &quot;'&quot; , &quot;);
extendedPersonNameDetails.appendChild(personType);
personType.appendChild(doc.createTextNode(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));

var isExtendedNameDetailsSupported = doc.createElement(&quot; , &quot;'&quot; , &quot;isExtendedNameDetailsSupported&quot; , &quot;'&quot; , &quot;);
extendedPersonNameDetails.appendChild(isExtendedNameDetailsSupported);
extendedPersonNameDetails.appendChild(doc.createTextNode(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));

var ssResults = main[&quot;invokeSS&quot;](&quot;C1-ChkExNmDt&quot;, doc, null, true, null);

var results = ssResults.getElementsByTagName(&quot; , &quot;'&quot; , &quot;isExtendedNameDetailsSupported&quot; , &quot;'&quot; , &quot;);

if (results[0].childNodes[0].nodeValue == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;) {
IS_EXT_NAME_SUPPORTED = &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;;
id(&quot; , &quot;'&quot; , &quot;tableRow1&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
} else {

id(&quot; , &quot;'&quot; , &quot;tableRow2&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
id(&quot; , &quot;'&quot; , &quot;tableRow3&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
id(&quot; , &quot;'&quot; , &quot;tableRow4&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;

}

} catch (err) {
alert(&quot; , &quot;'&quot; , &quot;DEBUG: &quot; , &quot;'&quot; , &quot; + err.description + error); //for debugging..
}
}

var oldOnload = window.onload;

function init() {
oldOnload();
}

window.onload = init;






   function getEntityNameFieldSize(){
       //Defines the standard Entity Name element size across UI-Maps
       return 80;
   }


   function getEntityNameMaxlength(){
        //Defines the standard Entity Name element size across UI-Maps
        return 254
   }

   function setTextFieldMaxlength(inputTextField){
    if(!inputTextField.getAttribute(&quot; , &quot;'&quot; , &quot;maxlength&quot; , &quot;'&quot; , &quot;)){
        addHTMLAttribute(inputTextField, &quot; , &quot;'&quot; , &quot;maxlength&quot; , &quot;'&quot; , &quot;,  getEntityNameMaxlength());
    }
   }

    function addHTMLAttribute( htmlElement, attribName, attribValue ){
        if(htmlElement){
            htmlElement.setAttribute(attribName, attribValue);
        }
    }

    function expandTextFieldWidth( elementId ){        
    
        var inputTextField = id(elementId)

        if(inputTextField){              
                addHTMLAttribute(inputTextField, &quot; , &quot;'&quot; , &quot;size&quot; , &quot;'&quot; , &quot;,  getEntityNameFieldSize());
        }

        setTextFieldMaxlength(inputTextField);

    }



Nome





Nome Completo





Primeiro Nome



Último Nome



Tipo de ID


Bilhete de IdentidadeAssento de NascimentoCertidão de NascimentoNº de Registo ComercialCarta de ConduçãoIdentificação de Cidadão EstrangeiroBI LegadoNIFOutroPassaporteIdentificação de Funcionário PúblicoCartão de RefugiadoCartão de ResidenteCartão de Eleitor
 Valor da ID 






999999999XX999, where 9&quot; , &quot;'&quot; , &quot;s repr








          window.filterMap1 = {};
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F1&quot; , &quot;'&quot; , &quot;] = &quot;type=string name likeable=PS label=FULL_NAME&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F2&quot; , &quot;'&quot; , &quot;] = &quot;type=string firstNameValue likeable=S label=FIRST_NAME&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F3&quot; , &quot;'&quot; , &quot;] = &quot;type=string lastNameValue likeable=S label=C1_LAST_NAME&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F4&quot; , &quot;'&quot; , &quot;] = &quot;type=string idType label=ID_TYPE_CD likeable=S&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F5&quot; , &quot;'&quot; , &quot;] = &quot;type=string idValue label=PER_ID_VAL&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F6&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F7&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F8&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F9&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F10&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F11&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F12&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F13&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F14&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F15&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F16&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F17&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F18&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F19&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F20&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F21&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F22&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F23&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F24&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F25&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          
      function refreshZone1(isFromAutoRefresh) {
        if(isFromAutoRefresh){
            initializeDEZone(&quot; , &quot;'&quot; , &quot;C1-TXFRQRY&quot; , &quot;'&quot; , &quot;, 1, isFromAutoRefresh, null, window.filterMap1);
        }else{
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, false);
      }
          
      } 
      function refreshZone1ForBroadcast() {
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, true);
      }
      function storeZoneParameters1(document, _portalState, isFromSavePref) {
          parent.storeDataExplorerZoneParameters(1, document, _portalState, isFromSavePref);
      }
      
       function refreshZone1ForPagination(pageDirection) {
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, false,pageDirection);
      }
     
     function getSearchButtonElement1(document){
      	   var inputSeq = 1;
           var idForSearchButton = &quot;anTLZ&quot; + inputSeq + &quot;Refresh&quot;;
		   var searchButton = id(idForSearchButton, document);
		   if (main.accessibilityFeatureEnabled &amp;&amp; !searchButton) {
		    	idForSearchButton = &quot;anTLZ&quot; + inputSeq + &quot;Refresh_dnd&quot;;
		    	searchButton = id(idForSearchButton, document);
		    	
		  }
		  return searchButton;
	}
     
      function positionSearchButton1() {
		    if (!isReadyStateComplete(window)) {
		        window.setTimeout(positionSearchButton1, 200);
		        return;
		    }
		    var inputSeq = 1;
		    var inputElem = getSearchButtonElement1(document);  
		    if (!inputElem) {
		        return;
		    }	
		    
		    var filterMSeq = 1;
		    var spanFilterId = &quot;zoneBody&quot; + filterMSeq;
		    var dataExplorerFiltersId = &quot;dataExplorerFilters&quot; + filterMSeq;
		    var multiQueryFilterTableId = &quot;multiQueryFilterBody&quot; + inputSeq; //multiQueryFilterBody{$sequenceId}
		
		    //pick last element from the filterUiMap
		    var lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + spanFilterId + &quot; :input[orafield]&quot;, document);
		    var lstLabel = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + spanFilterId + &quot; label[oraLabel], label[oraMdLabel]&quot;, document);
		
		    //now search the default filter elements from multiQueryFilterTable
		    if (lstElems.length == 0) {
		        lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + multiQueryFilterTableId + &quot; :input[id^=filter]&quot;, document); //elements whose id starting with filter
		    }
		    
		    if(lstElems.length == 0){
		    	lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + dataExplorerFiltersId + &quot; :input[id^=filter]&quot;, document);
		    }
		    
		    var isLTRdirection = true;
		     
		    var pos, i = 0, j = 0;
		    for (i = 0; i &lt; lstElems.length; i++) {
		    	if(!ouaf.core.CSSHelper.isElementHidden(lstElems[i])){
		    		if(isLTRdirection)
			        	pos =  lstElems[i].parentNode.offsetLeft;											
			        else
			        	pos = ouaf.dom.DOMHelper.getFarEnd(lstElems[i]);
			        if (pos > 0) {
			            break;
			        }
		        }
		    }
		    if(!pos){
			    for (j = 0; j &lt; lstLabel.length; j++) {
		    	    if(!ouaf.core.CSSHelper.isElementHidden(lstLabel[j])){
		    		    if(isLTRdirection)
							pos = ouaf.dom.DOMHelper.getFarEnd(lstLabel[j]); 												
						if (pos > 0) {
							break;
						}	                   
					}
				}
			}	
		    
		    ouaf.core.CSSHelper.removeClassByElement(inputElem, &quot;hide&quot;);//display the Search button before returning
		    if (i > lstElems.length || lstElems.length==0) return; //found no element in the list
		    
			if(!isLTRdirection &amp;&amp; ouaf.dom.DOMHelper.getFarEnd(inputElem) > pos){
				pos=ouaf.dom.DOMHelper.getFarEnd(inputElem)-pos;
				ouaf.core.CSSHelper.setStyleByElement(inputElem, {&quot;margin-right&quot;:  pos+&quot;px&quot;});	
			}else{	       
			    ouaf.core.CSSHelper.setStyleByElement(inputElem, {&quot;margin-left&quot;: pos + &quot;px&quot;});
		    }
		
		}
		window.setTimeout(positionSearchButton1, 200);
      
  




















































































          		var func1 = function(){
                	parent.handleDataExplorerZoneLoad(document, 1, true);
          		};
          		func1();
          








/html[1]&quot;) or . = concat(&quot;




var main = parent;
var myPos = parent;
var noOfZones = 1;
 var refreshPeriodList=[]; var zoneIntervals=[]; refreshPeriodList[1]=&quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;;var portalOptions={&quot;lists&quot;:{&quot;optionValues&quot;:{&quot;name&quot;:&quot;optionValues&quot;,&quot;header&quot;:{&quot;moreRows&quot;:false,&quot;remainingRows&quot;:0,&quot;lastInfo&quot;:&quot;&quot;},&quot;list&quot;:[]}}};var portalMOAndKeys = {};pushMOAndKeysIfAvailable();
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

main.pendingGlobalContext.waitForZone(0);



Pesquisa de Formulários Fiscais

 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField[&quot; , &quot;'&quot; , &quot;EMPTY_SUMMARY&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;\x22\x22&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_FILTERS_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Filtros&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_EXPLORER_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Explorador&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_GOTOZONE_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Ir\x20Para\x20Zona&quot; , &quot;'&quot; , &quot; };

 
  
           
          
      
     
  
 

  

   document.addEventListener(&quot;DOMContentLoaded&quot;, function() {
     checkMultiQueryFilterVisibility(id(&quot; , &quot;'&quot; , &quot;filterButton_1&quot; , &quot;'&quot; , &quot;, document), 1);
   });

   if (!this.handleDashboardLoad) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;zoneMenu_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   } 
 /* if (trimString(id(&quot; , &quot;'&quot; , &quot;helpText_1&quot; , &quot;'&quot; , &quot;, document).innerHTML) != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
    oraInsertHelpForZoneHeaders(1);    
  }*/
 
   checkMultiQueryFilterVisibility(id(&quot; , &quot;'&quot; , &quot;filterButton_1&quot; , &quot;'&quot; , &quot;, document), 1); 
   checkDnDVisibility(id(&quot; , &quot;'&quot; , &quot;dndButton_1&quot; , &quot;'&quot; , &quot;, document), 1); 

   if (id(&quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;, main.document)) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;editZone_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   }
     
   if (&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; != &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;exportZone_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   } 

var showSavePreferenes=false;
if((parent.main.userProfile.isTemplateUser == true) || (parent.main.userProfile.isTemplateUser == false &amp;&amp; parent.main.userProfile.portalProfileUserId == &quot;&quot;)){
showSavePreferenes= true;
}
 if(showSavePreferenes){
     var elem = id(&quot; , &quot;'&quot; , &quot;savePreferences_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
  }
  if(id(&quot; , &quot;'&quot; , &quot;resetZonePreferences_1&quot; , &quot;'&quot; , &quot;, document)){
     var elem = id(&quot; , &quot;'&quot; , &quot;resetZonePreferences_1&quot; , &quot;'&quot; , &quot;, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   }

function showMenu(evt, sequenceId) {
    if(window.currentSequence) 
        window.hideMenuImmediately(sequenceId);
    
    var menu = id(&quot; , &quot;'&quot; , &quot;explorerZoneMenu_&quot; , &quot;'&quot; , &quot; + sequenceId, document);
    setStyle(menu, &quot; , &quot;'&quot; , &quot;z-index&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;20&quot; , &quot;'&quot; , &quot;);
    
    if (id(&quot; , &quot;'&quot; , &quot;dataExplorerDescription1&quot; , &quot;'&quot; , &quot;, document)) { 
        setStyle(getFirstNonTextChild(menu).rows[3], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); 
    } 
    if (id(&quot; , &quot;'&quot; , &quot;debug&quot; , &quot;'&quot; , &quot;, main.document)) { 
        setStyle(getFirstNonTextChild(menu).rows[3], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); 
        setStyle(getFirstNonTextChild(menu).rows[4], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        setStyle(getFirstNonTextChild(menu).rows[5], &quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    } 
    setStyle(menu, &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, (main.computeAbsoluteOffsetTop(getTarget(evt)) + 5) + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
    setStyle(menu, &quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, (main.computeAbsoluteOffsetLeft(getTarget(evt)) + 5) + &quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
    ouaf.core.CSSHelper.removeClassByElement(menu,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
    window.currentSequence = sequenceId;
    function leave(e)
    {
        hideMenuImmediately(sequenceId);
    }
    
    if(!_IE) ffEmulator.addEvent(menu, &quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, leave, false);
    else addListener(id(&quot; , &quot;'&quot; , &quot;explorerZoneMenu_&quot; , &quot;'&quot; , &quot; + sequenceId, document), &quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, leave );
    
    if(_IE) showFrame(menu, sequenceId);
}

function checkMultiQueryFilterVisibility(element, sequenceId) { 
    var location = &quot;&quot;; 
    var filters = &quot;&quot;; 
    if ((location.toUpperCase() == &quot; , &quot;'&quot; , &quot;T&quot; , &quot;'&quot; , &quot;) 
       || (filters.toUpperCase().indexOf(&quot;FILTERAREA=N&quot;) > -1)  ) { 
        ouaf.core.CSSHelper.addClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
        return;
    } 
    if(window.filterMap1){
         var filter_string = window.filterMap1[&quot; , &quot;'&quot; , &quot;F1&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F2&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F3&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F4&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F5&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F6&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F7&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F8&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F9&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F10&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F11&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F12&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F13&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F14&quot; , &quot;'&quot; , &quot;] + window.filterMap1[&quot; , &quot;'&quot; , &quot;F15&quot; , &quot;'&quot; , &quot;];

         if(filter_string == &quot;&quot;)
           ouaf.core.CSSHelper.addClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
         else
           ouaf.core.CSSHelper.removeClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
    }
    else
           ouaf.core.CSSHelper.addClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
}

function checkDnDVisibility(element, sequenceId) { 
    var dnd = &quot;&quot;; 
    if (   (this.handleDashboardLoad)  
       || (dnd.toUpperCase().indexOf(&quot;DRAGDROPAREA=N&quot;) > -1)  
       ) { 
        return; 
    } 
    ouaf.core.CSSHelper.removeClassByElement(element,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
} 

// other related functions have been moved to userMapSupport.js


 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_EXPLORER_ZONE_MENU_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Menu\x20da\x20Zona\x20do\x20Explorador&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_OPEN_SAVED_SEARCH_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Open\x20Saved\x20Search\x2E\x2E\x2E&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_CLEAR_PREFERENCES&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Clear\x20Filters&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_EXP_TO_EXCEL_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Exportar\x20Para\x20Excel&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_PRINT_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Imprimir\x20Zona&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_SHOW_XML_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Mostrar\x20Dados\x20do\x20Serviço&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_SHOW_SQL_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Mostrar\x20SQL&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_SAVE&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Save&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_SAVE_AS&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Save\x20As\x20\x2E\x2E\x2E&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_DELETE&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Delete&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_SET_AS_DEFAULT&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Set\x20As\x20Default&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_REMOVE_AS_DEFAULT&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Remove\x20As\x20Default&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_ADD_TO_FAVORITES&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Add\x20To\x20Favorites&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;F1_REMOVE_FROM_FAVORITES&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Remove\x20From\x20Favorites&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_PRINT_LBL_STANDARD&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Standard&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;DE_PRINT_LBL_FORMATTED&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Formatted&quot; , &quot;'&quot; , &quot; };

 main.zoneMenuWindow = getDefaultView(document); 

    

   
 
	 
	 
               Open Saved Search...
			   
					
			   
			   
     
     
               Clear Filters
			   
			   
     
  
	  
			 Exportar Para Excel

			   			 
	  
		 
      
         Imprimir Zona
		 
                       
			   
      
	 
      
         Mostrar Dados do Serviço
		 
	         
      
	 
      
         Mostrar SQL 
		 
			   
      
	  
	  
         Save
		 
			   
      
	  
         Save As ...
		 
			   
      
	  
         Delete 
		 
			   
      
	  
         Set As Default
		 
			   
      
	  
         Remove As Default
		 
			   
      
	  
         Add To Favorites
		 
			   
      
      
	  
         Remove From Favorites
		 
			   
      







 
      
         Standard
		 
			   
      

      
         Formatted
		 
			   
      






var loadedSavedSearchName_1;


function lazyLoad_1(seqNumber, rowsToExport) {
    if (!this.handleDashboardLoad) {
        var elem = id(&quot; , &quot;'&quot; , &quot;zoneMenu_1&quot; , &quot;'&quot; , &quot;, document);
        ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
    } 
    if (trimString(id(&quot; , &quot;'&quot; , &quot;helpText_1&quot; , &quot;'&quot; , &quot;, document).innerHTML) != &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
        main.zoneMenuWindow.oraInsertHelpForZoneHeaders(1);    
    }
    else {
        main.zoneMenuWindow.oraInsertSavedSearchTitle(null,1);  
    }
    updateZoneMenuHTML(seqNumber, rowsToExport);
}

setTimeout(lazyLoad_1, 200);
setTimeout(lazyLoad_1(1,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;), 200);

 
function buildNameSearch_1(document,sequenceId){
var savedSearchNames =getDefaultView(document)[&quot; , &quot;'&quot; , &quot;savedSearches&quot; , &quot;'&quot; , &quot;+sequenceId];
var innerListItems =&quot;&quot;;
  if(savedSearchNames){
             if (savedSearchNames.length > 0) {

   	       for(var i=0;i&lt;savedSearchNames.length;i++){
		 var searchName=savedSearchNames[i].name;
		 if(searchName){
  		   var lineItem=&quot;&lt;li id=\&quot;savedItem_&quot;+i+&quot;\&quot; tabindex=\&quot;5\&quot; data-menuType=\&quot;HTML\&quot; class=\&quot;menuItem\&quot; role=\&quot;menuitem\&quot; tabIndex=\&quot;5\&quot; onclick=\&quot;ouaf.context.expandSubMenu(true); main.zoneMenuWindow.loadNameSearch(main.zoneMenuWindow.document,1); main.zoneMenuWindow.hideMenuImmediately(1); return false;\&quot;  onkeypress=\&quot; ouaf.context.onMenuKeyPress(event); \&quot; onkeydown=\&quot;ouaf.context.onMenuKeyPress(event)\&quot; onmouseover=\&quot;ouaf.context.expandSubMenu(false)\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot; data-isLeaf=\&quot;true\&quot;>&quot;;
                   if (savedSearchNames[i].isDefault) {
						lineItem = lineItem+&quot;&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;img tabindex=\&quot;5\&quot;  id=\&quot;defaultSearch_1\&quot; src=\&quot;images/star.gif\&quot; />&quot;+searchName+&quot;&lt;/span>&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;/li>&quot;;
                   }else{
						lineItem = lineItem+&quot;&lt;span  data-menuType=\&quot;HTML\&quot; data-menuObj = \&quot;CI_ZONEMENU\&quot;>&quot;+searchName+&quot;&lt;/span>&lt;span  data-menuObj = \&quot;CI_ZONEMENU\&quot;>&lt;/li>&quot;;
				   }
                   
   	           innerListItems = innerListItems + lineItem;
	    	 }
               }
             } else {
                var elem = id(&quot; , &quot;'&quot; , &quot;savedSearch_1&quot; , &quot;'&quot; , &quot;,document);
                ouaf.core.CSSHelper.addClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
             }
  } else {
                var elem = id(&quot; , &quot;'&quot; , &quot;savedSearch_1&quot; , &quot;'&quot; , &quot;,document);
                ouaf.core.CSSHelper.addClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
  }
  if(document.getElementById(&quot; , &quot;'&quot; , &quot;savedSearchSubMenuList_&quot; , &quot;'&quot; , &quot;+sequenceId)){
    document.getElementById(&quot; , &quot;'&quot; , &quot;savedSearchSubMenuList_&quot; , &quot;'&quot; , &quot;+sequenceId).innerHTML = innerListItems;
  }
} 


function updateZoneMenuHTML(seqNum, rowsToExport){
   if (rowsToExport != &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot;) { 
     var elem = id(&quot; , &quot;'&quot; , &quot;exportZone_&quot; , &quot;'&quot; , &quot;+ seqNum, document); 
     ouaf.core.CSSHelper.removeClassByElement(elem,&quot; , &quot;'&quot; , &quot;hide&quot; , &quot;'&quot; , &quot;);
   } 
}   

function showMenuFusion_1(evt, sequenceId) {
  main.doClose();
  main.zoneMenuWindow = getDefaultView(document);
  main.applyMenuItemsDisplaySettings(document, sequenceId);
  var explZoneMenu = main.ouaf.ui.Menu({&quot;menuType&quot;:&quot;HTML&quot;, &quot;menuName&quot;:&quot;CI_ZONEMENU&quot;, &quot;menuModel&quot;:{&quot;html&quot;: document.getElementById(&quot;explorerZoneMenu_1&quot;).innerHTML}, &quot;savedSearchModel_1&quot;: {&quot;html&quot;: document.getElementById(&quot; , &quot;'&quot; , &quot;savedSearchesSubmenu_1&quot; , &quot;'&quot; , &quot;).innerHTML}, &quot;printMenuModel_1&quot;: {&quot;html&quot;: document.getElementById(&quot; , &quot;'&quot; , &quot;printSubmenu_1&quot; , &quot;'&quot; , &quot;).innerHTML}});
  main.menuObjectsMap[explZoneMenu.name]= explZoneMenu;
  
  explZoneMenu.toggle(evt);
  
}

  function clearLoadedSavedSearchName_1(sequenceId) {
    main.top.tabPage()[&quot; , &quot;'&quot; , &quot;loadedSavedSearchName_&quot; , &quot;'&quot; , &quot;+sequenceId] = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
  }

  document.addEventListener(&quot;DOMContentLoaded&quot;, function(event) {
        var buildMenuFunction = getDefaultView(document)[&quot; , &quot;'&quot; , &quot;buildNameSearch_1&quot; , &quot;'&quot; , &quot;];
        if (buildMenuFunction) {
            buildMenuFunction(document, 1);
        }
  });
Acrescentar


 
 var userZonePreferences1={};
var savedSearches1=[];




      var sqlData = { 
    A: {C1: { type:  
    &quot; , &quot;'&quot; , &quot;D&quot; , &quot;'&quot; , &quot;
   
            , width: 0
            }
       }
   }; 
    














Número da Mensagem
                     ,










Não existem informações para apresentar





Pesquisa por
      
         Pessoa
	Data Receção / Tipo Formul. / Estado / Tipo Declaração
	Endereço
	ID Batch Formulários Externo, Cabeçalho Batch Formulários
	Localização do Documento
	







    var oraSchemaInfo2={name:&quot; , &quot;'&quot; , &quot;CM\x2DFormQueryFilter8&quot; , &quot;'&quot; , &quot;,type:&quot; , &quot;'&quot; , &quot;F1MP&quot; , &quot;'&quot; , &quot;,children:[{name:&quot; , &quot;'&quot; , &quot;entityNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;ENTITY\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:254},{name:&quot; , &quot;'&quot; , &quot;fullNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;FULL\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome\x20Completo&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:254},{name:&quot; , &quot;'&quot; , &quot;firstNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;C1\x5FFIRST\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome\x20Próprio&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:50},{name:&quot; , &quot;'&quot; , &quot;lastNameValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;C1\x5FLAST\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Apelido&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:50},{name:&quot; , &quot;'&quot; , &quot;idType&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;ID\x5FTYPE\x5FCD&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Tipo\x20de\x20Identificação&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:8},{name:&quot; , &quot;'&quot; , &quot;idValue&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;PER\x5FID\x5FVAL&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Valor\x20da\x20ID&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:16},{name:&quot; , &quot;'&quot; , &quot;personType&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;\x28personType\x29&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;},{name:&quot; , &quot;'&quot; , &quot;isExtendedNameDetailsSupported&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;\x28isExtendedNameDetailsSupported\x29&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;boolean&quot; , &quot;'&quot; , &quot;},{name:&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;,mdField:&quot; , &quot;'&quot; , &quot;ENTITY\x5FNAME&quot; , &quot;'&quot; , &quot;,label:&quot; , &quot;'&quot; , &quot;Nome&quot; , &quot;'&quot; , &quot;,dataType:&quot; , &quot;'&quot; , &quot;string&quot; , &quot;'&quot; , &quot;,precision:254}]}


function table_CIPCIDFW() {
	return [
		{key: &quot;AID&quot;, value: &quot;999999999XX999\x2C\x20where\x209\x27s\x20repr&quot;}
	];
} 
 document.table_CIPCIDFW = table_CIPCIDFW();

 if(!oraMdField) {
	 var oraMdField = {};
 }
    oraMdField[&quot; , &quot;'&quot; , &quot;ENTITY_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Nome&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;FULL_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Nome\x20Completo&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;FIRST_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Primeiro\x20Nome&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;LAST_NAME&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Último\x20Nome&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;ID_TYPE_LBL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Tipo\x20de\x20ID&quot; , &quot;'&quot; , &quot; };
    oraMdField[&quot; , &quot;'&quot; , &quot;PER_ID_VAL&quot; , &quot;'&quot; , &quot;]={ fieldValue: &quot; , &quot;'&quot; , &quot;Valor\x20da\x20ID&quot; , &quot;'&quot; , &quot; };



var FORMAT_ID_JS_FILENAME = &quot;code/formatValue.js&quot;;
var IS_EXT_NAME_SUPPORTED = &quot;false&quot;;

function oraLoad1() {

/* load FW formatValue.js dynamically since using script tag
* and pointing it to a JS source is not allowed in fragment
* UI Maps
*/
var fileref = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;);
fileref.setAttribute(&quot;type&quot;, &quot;text/javascript&quot;);
fileref.setAttribute(&quot;src&quot;, FORMAT_ID_JS_FILENAME);
id(&quot; , &quot;'&quot; , &quot;filterUIFields&quot; , &quot;'&quot; , &quot;).appendChild(fileref);
invokeSS();

}


function formatIdNumber() {
var personIdNbr = id(&quot; , &quot;'&quot; , &quot;personIdValue&quot; , &quot;'&quot; , &quot;).value;
var idTypeCode = id(&quot; , &quot;'&quot; , &quot;personIdType&quot; , &quot;'&quot; , &quot;).value;
var idFormat = &quot;&quot;;
if (personIdNbr == &quot;&quot; || idTypeCode == &quot;&quot;) {
return;
}
for (var i = 0; i &lt; document.table_CIPCIDFW.length; i++) {

if (document.table_CIPCIDFW[i].key == idTypeCode) {
idFormat = document.table_CIPCIDFW[i].value;
break;
}
}
id(&quot; , &quot;'&quot; , &quot;personIdValue&quot; , &quot;'&quot; , &quot;).value = formatValue(personIdNbr, idFormat);
}

function updateName() {
if (IS_EXT_NAME_SUPPORTED == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;)
id(&quot; , &quot;'&quot; , &quot;nameValue&quot; , &quot;'&quot; , &quot;).value = id(&quot; , &quot;'&quot; , &quot;fullNameValue&quot; , &quot;'&quot; , &quot;).value;
else
id(&quot; , &quot;'&quot; , &quot;nameValue&quot; , &quot;'&quot; , &quot;).value = id(&quot; , &quot;'&quot; , &quot;entityNameValue&quot; , &quot;'&quot; , &quot;).value;
}


function invokeSS() {

var error = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
try {
var doc = createXMLDoc(null, null, selectionNamespaces);
var extendedPersonNameDetails = doc.createElement(&quot; , &quot;'&quot; , &quot;extendedPersonNameDetails&quot; , &quot;'&quot; , &quot;);
doc.appendChild(extendedPersonNameDetails);
extendedPersonNameDetails.setAttribute(&quot; , &quot;'&quot; , &quot;type&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;group&quot; , &quot;'&quot; , &quot;);

var personType = doc.createElement(&quot; , &quot;'&quot; , &quot;personType&quot; , &quot;'&quot; , &quot;);
extendedPersonNameDetails.appendChild(personType);
personType.appendChild(doc.createTextNode(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));

var isExtendedNameDetailsSupported = doc.createElement(&quot; , &quot;'&quot; , &quot;isExtendedNameDetailsSupported&quot; , &quot;'&quot; , &quot;);
extendedPersonNameDetails.appendChild(isExtendedNameDetailsSupported);
extendedPersonNameDetails.appendChild(doc.createTextNode(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;));

var ssResults = main[&quot;invokeSS&quot;](&quot;C1-ChkExNmDt&quot;, doc, null, true, null);

var results = ssResults.getElementsByTagName(&quot; , &quot;'&quot; , &quot;isExtendedNameDetailsSupported&quot; , &quot;'&quot; , &quot;);

if (results[0].childNodes[0].nodeValue == &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;) {
IS_EXT_NAME_SUPPORTED = &quot; , &quot;'&quot; , &quot;true&quot; , &quot;'&quot; , &quot;;
id(&quot; , &quot;'&quot; , &quot;tableRow1&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
} else {

id(&quot; , &quot;'&quot; , &quot;tableRow2&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
id(&quot; , &quot;'&quot; , &quot;tableRow3&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;
id(&quot; , &quot;'&quot; , &quot;tableRow4&quot; , &quot;'&quot; , &quot;).style.display = &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;;

}

} catch (err) {
alert(&quot; , &quot;'&quot; , &quot;DEBUG: &quot; , &quot;'&quot; , &quot; + err.description + error); //for debugging..
}
}

var oldOnload = window.onload;

function init() {
oldOnload();
}

window.onload = init;






   function getEntityNameFieldSize(){
       //Defines the standard Entity Name element size across UI-Maps
       return 80;
   }


   function getEntityNameMaxlength(){
        //Defines the standard Entity Name element size across UI-Maps
        return 254
   }

   function setTextFieldMaxlength(inputTextField){
    if(!inputTextField.getAttribute(&quot; , &quot;'&quot; , &quot;maxlength&quot; , &quot;'&quot; , &quot;)){
        addHTMLAttribute(inputTextField, &quot; , &quot;'&quot; , &quot;maxlength&quot; , &quot;'&quot; , &quot;,  getEntityNameMaxlength());
    }
   }

    function addHTMLAttribute( htmlElement, attribName, attribValue ){
        if(htmlElement){
            htmlElement.setAttribute(attribName, attribValue);
        }
    }

    function expandTextFieldWidth( elementId ){        
    
        var inputTextField = id(elementId)

        if(inputTextField){              
                addHTMLAttribute(inputTextField, &quot; , &quot;'&quot; , &quot;size&quot; , &quot;'&quot; , &quot;,  getEntityNameFieldSize());
        }

        setTextFieldMaxlength(inputTextField);

    }



Nome





Nome Completo





Primeiro Nome



Último Nome



Tipo de ID


Bilhete de IdentidadeAssento de NascimentoCertidão de NascimentoNº de Registo ComercialCarta de ConduçãoIdentificação de Cidadão EstrangeiroBI LegadoNIFOutroPassaporteIdentificação de Funcionário PúblicoCartão de RefugiadoCartão de ResidenteCartão de Eleitor
 Valor da ID 






999999999XX999, where 9&quot; , &quot;'&quot; , &quot;s repr








          window.filterMap1 = {};
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F1&quot; , &quot;'&quot; , &quot;] = &quot;type=string name likeable=PS label=FULL_NAME&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F2&quot; , &quot;'&quot; , &quot;] = &quot;type=string firstNameValue likeable=S label=FIRST_NAME&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F3&quot; , &quot;'&quot; , &quot;] = &quot;type=string lastNameValue likeable=S label=C1_LAST_NAME&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F4&quot; , &quot;'&quot; , &quot;] = &quot;type=string idType label=ID_TYPE_CD likeable=S&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F5&quot; , &quot;'&quot; , &quot;] = &quot;type=string idValue label=PER_ID_VAL&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F6&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F7&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F8&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F9&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F10&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F11&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F12&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F13&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F14&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F15&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F16&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F17&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F18&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F19&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F20&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F21&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F22&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F23&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F24&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          window.filterMap1[&quot; , &quot;'&quot; , &quot;F25&quot; , &quot;'&quot; , &quot;] = &quot;&quot;;
          
      function refreshZone1(isFromAutoRefresh) {
        if(isFromAutoRefresh){
            initializeDEZone(&quot; , &quot;'&quot; , &quot;C1-TXFRQRY&quot; , &quot;'&quot; , &quot;, 1, isFromAutoRefresh, null, window.filterMap1);
        }else{
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, false);
      }
          
      } 
      function refreshZone1ForBroadcast() {
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, true);
      }
      function storeZoneParameters1(document, _portalState, isFromSavePref) {
          parent.storeDataExplorerZoneParameters(1, document, _portalState, isFromSavePref);
      }
      
       function refreshZone1ForPagination(pageDirection) {
          parent.dataExplorerRefreshZone(document, 1, true, true, window.filterMap1, false,pageDirection);
      }
     
     function getSearchButtonElement1(document){
      	   var inputSeq = 1;
           var idForSearchButton = &quot;anTLZ&quot; + inputSeq + &quot;Refresh&quot;;
		   var searchButton = id(idForSearchButton, document);
		   if (main.accessibilityFeatureEnabled &amp;&amp; !searchButton) {
		    	idForSearchButton = &quot;anTLZ&quot; + inputSeq + &quot;Refresh_dnd&quot;;
		    	searchButton = id(idForSearchButton, document);
		    	
		  }
		  return searchButton;
	}
     
      function positionSearchButton1() {
		    if (!isReadyStateComplete(window)) {
		        window.setTimeout(positionSearchButton1, 200);
		        return;
		    }
		    var inputSeq = 1;
		    var inputElem = getSearchButtonElement1(document);  
		    if (!inputElem) {
		        return;
		    }	
		    
		    var filterMSeq = 1;
		    var spanFilterId = &quot;zoneBody&quot; + filterMSeq;
		    var dataExplorerFiltersId = &quot;dataExplorerFilters&quot; + filterMSeq;
		    var multiQueryFilterTableId = &quot;multiQueryFilterBody&quot; + inputSeq; //multiQueryFilterBody{$sequenceId}
		
		    //pick last element from the filterUiMap
		    var lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + spanFilterId + &quot; :input[orafield]&quot;, document);
		    var lstLabel = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + spanFilterId + &quot; label[oraLabel], label[oraMdLabel]&quot;, document);
		
		    //now search the default filter elements from multiQueryFilterTable
		    if (lstElems.length == 0) {
		        lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + multiQueryFilterTableId + &quot; :input[id^=filter]&quot;, document); //elements whose id starting with filter
		    }
		    
		    if(lstElems.length == 0){
		    	lstElems = ouaf.dom.DOMHelper.getElements(&quot;#&quot; + dataExplorerFiltersId + &quot; :input[id^=filter]&quot;, document);
		    }
		    
		    var isLTRdirection = true;
		     
		    var pos, i = 0, j = 0;
		    for (i = 0; i &lt; lstElems.length; i++) {
		    	if(!ouaf.core.CSSHelper.isElementHidden(lstElems[i])){
		    		if(isLTRdirection)
			        	pos =  lstElems[i].parentNode.offsetLeft;											
			        else
			        	pos = ouaf.dom.DOMHelper.getFarEnd(lstElems[i]);
			        if (pos > 0) {
			            break;
			        }
		        }
		    }
		    if(!pos){
			    for (j = 0; j &lt; lstLabel.length; j++) {
		    	    if(!ouaf.core.CSSHelper.isElementHidden(lstLabel[j])){
		    		    if(isLTRdirection)
							pos = ouaf.dom.DOMHelper.getFarEnd(lstLabel[j]); 												
						if (pos > 0) {
							break;
						}	                   
					}
				}
			}	
		    
		    ouaf.core.CSSHelper.removeClassByElement(inputElem, &quot;hide&quot;);//display the Search button before returning
		    if (i > lstElems.length || lstElems.length==0) return; //found no element in the list
		    
			if(!isLTRdirection &amp;&amp; ouaf.dom.DOMHelper.getFarEnd(inputElem) > pos){
				pos=ouaf.dom.DOMHelper.getFarEnd(inputElem)-pos;
				ouaf.core.CSSHelper.setStyleByElement(inputElem, {&quot;margin-right&quot;:  pos+&quot;px&quot;});	
			}else{	       
			    ouaf.core.CSSHelper.setStyleByElement(inputElem, {&quot;margin-left&quot;: pos + &quot;px&quot;});
		    }
		
		}
		window.setTimeout(positionSearchButton1, 200);
      
  




















































































          		var func1 = function(){
                	parent.handleDataExplorerZoneLoad(document, 1, true);
          		};
          		func1();
          








/html[1]&quot;))]</value>
      <webElementGuid>fe0c53f5-9647-4f15-b53d-f31bc68f5af5</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
