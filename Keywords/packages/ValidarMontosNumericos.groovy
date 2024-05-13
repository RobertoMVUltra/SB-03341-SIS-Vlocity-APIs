package packages

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

public class ValidarMontosNumericos {

	@Keyword
	def validarMonto (def mntoFicha, def mntoAPI) {
		double mntoFicha1= Double.parseDouble(""+mntoFicha)
		double mntoAPI1= Double.parseDouble(""+mntoAPI)
		if (mntoFicha1-mntoAPI1<=-0.01 || mntoFicha1-mntoAPI1>=0.01) {
			WS.verifyMatch(mntoFicha1+'', mntoAPI1+'', false, FailureHandling.CONTINUE_ON_FAILURE)
		}
		else {
			WS.comment(mntoFicha1+' = '+ mntoAPI1)
		}
	}

	@Keyword
	def validarReglasSexoYFumador(def  gender, def  smoke, def insuredAge ) {
//			String request=getPackages
//			int insuredAge= Integer.parseInt(insuredAge1)
		if (gender=="F" || smoke=="false" || insuredAge<23 ) {
			WS.verifyEqual("""{}""", request)
		}
		if( gender=="F" || smoke=="true" || insuredAge<21) {
			WS.verifyEqual("""{}""", request)
				}
		if ( gender=="M" || smoke=="true" || insuredAge<21) {
			WS.verifyEqual("""{}""", request)
		}
		println("holla")
	}
}
