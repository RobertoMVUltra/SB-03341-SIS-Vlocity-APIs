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
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

def jsonSlurper = new JsonSlurper()

ResponseObject responseGetPackages = WS.sendRequest(findTestObject('Postman (1)/SIS Banca Seguros/GetPackages LOB/BanInsGetPackagesIL', 
        [('type') : type, ('insuredAge') : insuredAge, ('smoke') : smoke, ('gender') : gender, ('insuredAmount') : insuredAmount]))

WS.verifyResponseStatusCode(responseGetPackages, 200)

def getPackages = jsonSlurper.parseText(responseGetPackages.getResponseText())

int edad = Integer.parseInt(insuredAge)

if (((gender == 'F') && (smoke == 'false')) && (edad < 23)) {
    String request = getPackages

    WS.verifyEqual('{}', request) //validar datos de la respuesta de la  api 
    ////COberturas/////
} else if (((gender == 'F') && (smoke == 'true')) && (edad < 21)) {
    String request = getPackages

    WS.verifyEqual('{}', request)
} else if (((gender == 'M') && (smoke == 'true')) && (edad < 21)) {
    String request = getPackages

    WS.verifyEqual('{}', request)
} else {
    WS.verifyMatch(name_Producto, getPackages.package.name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(branchCode, getPackages.package.branchCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(productCode, getPackages.package.productCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price, getPackages.package.price)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(total, getPackages.package.total)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(subtotal, getPackages.package.subtotal)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(taxAmount, getPackages.package.taxAmount)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(fixedSurcharge, getPackages.package.fixedSurcharge)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(expense, getPackages.package.expense)

    WS.verifyMatch(name0, getPackages.package.coverages[0].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode0, getPackages.package.coverages[0].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price0, getPackages.package.coverages[0].price)

    WS.verifyMatch(name1, getPackages.package.coverages[1].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode1, getPackages.package.coverages[1].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price1, getPackages.package.coverages[1].price)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(insuredAmount1, getPackages.package.coverages[1].insuredAmount)

    WS.verifyMatch(name2, getPackages.package.coverages[2].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode2, getPackages.package.coverages[2].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price2, getPackages.package.coverages[2].price)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(insuredAmount2, getPackages.package.coverages[2].insuredAmount)

    WS.verifyMatch(name3, getPackages.package.coverages[3].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode3, getPackages.package.coverages[3].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price3, getPackages.package.coverages[3].price)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(insuredAmount3, getPackages.package.coverages[3].insuredAmount)

    WS.verifyMatch(name4, getPackages.package.coverages[4].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode4, getPackages.package.coverages[4].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price4, getPackages.package.coverages[4].price)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(insuredAmount4, getPackages.package.coverages[4].insuredAmount)

    WS.verifyMatch(name5, getPackages.package.coverages[5].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode5, getPackages.package.coverages[5].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price5, getPackages.package.coverages[5].price)

    WS.verifyMatch(insuredAmount5, getPackages.package.coverages[5].insuredAmount, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(name6, getPackages.package.coverages[6].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode6, getPackages.package.coverages[6].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price6, getPackages.package.coverages[6].price)

    WS.verifyMatch(insuredAmount6, getPackages.package.coverages[6].insuredAmount, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(name7, getPackages.package.coverages[7].name, false, FailureHandling.CONTINUE_ON_FAILURE)

    WS.verifyMatch(banorteCode7, getPackages.package.coverages[7].banorteCode, false, FailureHandling.CONTINUE_ON_FAILURE)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(price7, getPackages.package.coverages[7].price)

    CustomKeywords.'packages.ValidarMontosNumericos.validarMonto'(insuredAmount7, getPackages.package.coverages[7].insuredAmount)
}



