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
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

WebUI.callTestCase(findTestCase('Front Office/CT - Iniciar Sessao'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/FRONT-OFFICE/Page_SIGT - Sistema Integrado de Gesto Tributria/a_Ajuda'))

WebUI.click(findTestObject('Object Repository/FRONT-OFFICE/Page_SIGT - Sistema Integrado de Gesto Tributria/a_Guias Rpidos'))

WebUI.click(findTestObject('Object Repository/FRONT-OFFICE/Page_SIGT - Sistema Integrado de Gesto Tributria/a_Gesto de Patrimnio'))

WebUI.click(findTestObject('Object Repository/FRONT-OFFICE/Page_SIGT - Sistema Integrado de Gesto Tributria/h4_Gesto de Patrimnio  Consulta de Registo _46a9f4'))

WebUI.click(findTestObject('Object Repository/FRONT-OFFICE/Page_SIGT - Sistema Integrado de Gesto Tributria/i_Baixar PDF_fa fa-angle-right'))

WebUI.delay(10)

WebUI.closeBrowser()

