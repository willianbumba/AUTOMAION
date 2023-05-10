import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://10.129.105.62:6501/ouaf/loginPage.jsp')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Entrada em Sesso/input_ID de Utilizador_j_username'), 'ILIDIO MARTINS')

WebUI.setEncryptedText(findTestObject('Object Repository/SIGT/Page_Entrada em Sesso/input_Palavra-passe_j_password'), '5xx1bkCcAlw=')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Entrada em Sesso/input_Palavra-passe_loginButton'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/span_Menu'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/span_Formularios'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/span_Formulrio de Cadastro'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/span_Adicionar'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_NormalOficiosa'), 'OFF', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_COLECTIVOSINGULAR'), 'COLLECTIVE', 
    true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_ASSOCIAOEMPRESAINSTITUIO PBLICAORGAN_6688db'), 
    'COM', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_Regime GeralRegime Simplificado'), 
    'GNAD', true)

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Denominao_asCurrent'), 'SOCIEDADE CHICHINO LDA')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Designao Comercial_asCurrent'), ' CHICHINO')

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_Administrao ou Autarquia MunicipalAd_7d0fdd'), 
    'ANON', true)

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/img_Beneficirio Efectivo_boGroup_collective_b4e6cd'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_Conta_anTLZ1Refresh'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/span_000079295LA010'))

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Beneficirio Efectivo_asCurrent'), '000079295LA010')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Publicado no Dirio da Repblica_asCurrent'), 
    'NA')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N do Dirio da Repblica_asCurrent'), 
    '00000')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Data da Publicao no Dirio da Repblica_a6b041'), 
    '24-11-2010')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Data da Constituio_asCurrent'), '18-10-2010')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Capital Social (AOA)_asCurrent'), '80.000,00')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Privado Nacional_asCurrent'), '100')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Pblico_asCurrent'), '0')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Estrangeiro_asCurrent'), '0')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Trabalhadores_asCurrent'), '4')

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/img_NIF_boGroup_otherCollectiveDetails_othe_4a89d1'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/select_Bilhete de IdentidadeAssento de Nasc_b550c9'), 
    'LNIF', true)

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_N Identificao_idValue'), 
    '003638974BE039')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_Conta_anTLZ1Refresh'))

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_NIF_asCurrent'), '003638974BE039')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/td_AdidoAdministradorCoordenadorCnsulDirect_984b5b'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AdidoAdministradorCoordenadorCnsulDi_49988a'), 
    'MANP', true)

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_NIF_asCurrent_1'))

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/img_NIF_otherCollNIF_asCurrent_search_0'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/select_Bilhete de IdentidadeAssento de Nasc_b550c9'), 
    'AID', true)

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_N Identificao_idValue'), 
    '003638974BE039')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_Conta_anTLZ1Refresh'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/select_Bilhete de IdentidadeAssento de Nasc_b550c9'), 
    'LNIF', true)

WebUI.click(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_Conta_anTLZ1Refresh'))

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_NIF_asCurrent_1'), '003638974BE039')

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/img_NIF_boGroup_otherCollectiveDetails_repr_3488fa'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/select_Bilhete de IdentidadeAssento de Nasc_b550c9'), 
    'LNIF', true)

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_N Identificao_idValue'), 
    '003638974BE039')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Search for Person Id, Accounts and NIF/input_Conta_anTLZ1Refresh'))

WebUI.switchToWindowTitle('Pesquisa a 360 Graus')

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_NIF_asCurrent_1_2'), '003638974BE039')

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AdministradorDirectorGerente Ou Gest_ce9ace'), 
    'MANG', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_EstrangeiroFixoTelemvel'), 
    'MOBILE', true)

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Nmero_asCurrent'), '923000000')

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_DeclarativoEstimativa'), 
    'EST', true)

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/tbody_Volume de NegciosTipoDeclarativoEstim_db3a4f'))

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Volume de Vendas_asCurrent'), '62.102.953,16')

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_AnualMensalSemestralTrimestral'), 
    'ANN', true)

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'))

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_N de Identificao Bancria (IBAN)_asCurrent'), 
    'AO06001005210137580401198')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_IBAN Principal_asCurrent'))

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/select_BANCO ANGOLANO DE INVESTIMENTOS, S.A_409fb6'), 
    'BPC', true)

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'))

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Conta_asCurrent'), '00')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'))

WebUI.setText(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Cdigo Swift_asCurrent'), '00')

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

WebUI.click(findTestObject('Object Repository/SIGT/Page_Pesquisa a 360 Graus/input_Iniciar Regime de IVA_asCurrent'))

