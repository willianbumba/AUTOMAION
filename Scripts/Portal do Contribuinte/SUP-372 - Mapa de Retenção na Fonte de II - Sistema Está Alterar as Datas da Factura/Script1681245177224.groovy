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

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.URL)

WebUI.setText(findTestObject('Object Repository/Portal do Contribuinte/Page_Iniciar Sesso Portal do Contribuinte/input_Username_j_username'), 
    utilizador)

WebUI.setText(findTestObject('Object Repository/Portal do Contribuinte/Page_Iniciar Sesso Portal do Contribuinte/input_Password_j_password'), 
    senha)

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Iniciar Sesso Portal do Contribuinte/input_Password_btnLoginDC'))

WebUI.setText(findTestObject('Object Repository/Portal do Contribuinte/Page_Empresas instituies do Contribuinte - _8be543/input_Filter by NIF_frmRepresentationsdtRep_73fa99'), 
    '5417040231')

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Empresas instituies do Contribuinte - _8be543/span_Seleccionar'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Extracto de Conta Corrente  Portal do _c5312d/span_Declaraes'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Extracto de Conta Corrente  Portal do _c5312d/span_II'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Extracto de Conta Corrente  Portal do _c5312d/span_Entregar'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Extracto de Conta Corrente  Portal do _c5312d/span_Mapa Reteno Fonte'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/button_editar'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/span_Editar'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/span_Adicionar Linha'))

WebUI.setText(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/input_Nmero Fiscal_formAddRecipientnifRecipient'), 
    '5417040231')

WebUI.sendKeys(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/input_Nmero Fiscal_formAddRecipientnifRecipient'), 
    Keys.chord(Keys.TAB))

WebUI.click(findTestObject('Portal do Contribuinte/Page_Portal do Contribuinte/input_Nmero da Factura_formAddRecipientinpu_cead7b'))

WebUI.setText(findTestObject('Portal do Contribuinte/Page_Portal do Contribuinte/input_Nmero da Factura_formAddRecipientinpu_cead7b'), 
    'FT999999')

WebUI.setText(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/input_Descrio do Servio_formAddRecipientj_id_g5'), 
    'TESTESTE')

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/input_Data da Factura_formAddRecipientj_id__d3bafe'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/span_Anterior'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/a_23'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/input_Data do Pagamento_formAddRecipientj_i_abadf9'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/a_2'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/span_Adicionar'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/td_23012023'))

WebUI.click(findTestObject('Object Repository/Portal do Contribuinte/Page_Portal do Contribuinte/td_02022023'))

WebUI.takeFullPageScreenshot()

WebUI.closeBrowser()

