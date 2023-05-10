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

/**
 * 
 * @author jessica-silva
 * Passos
 * P1: Aceder em: https://portaldocontribuinte.minfin.gov.ao/management-user/login
 * P2: Utilizador: Produção
 * P3: Gestão de Utilizadores → Gestão de Pedidos → Indicar NIF a Pesquisar
 * P4: Clicar no botão Visualizar
 *
 */

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.URL)

WebUI.setText(findTestObject('Portal Administrativo/Page_SIGT-Login-Sistema Integrado de Gesto _a92a66/input_Username_j_username'), utilizador)

WebUI.setText(findTestObject('Portal Administrativo/Page_SIGT-Login-Sistema Integrado de Gesto _a92a66/input_Password_j_password'), senha)

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_SIGT-Login-Sistema Integrado de Gesto _a92a66/input_Password_j_id_s'))

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/p_Gesto de Utilizadores'))

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/p_Gesto de Pedidos'))

WebUI.setText(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/input_NIF_frmSearchRegisterinput-nif'), '005858180NE049')

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/body_Toggle navigationSairAdministrao Geral_640c53'))

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/span_Pesquisar'))

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/td_005858180NE049'))

WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_ADM - GESTO ACESSO/span_visualizar'))

//WebUI.switchToWindowTitle('500 Internal Server Error')

//WebUI.click(findTestObject('Object Repository/Portal Administrativo/Page_500 Internal Server Error/h1_Internal Server Error'))